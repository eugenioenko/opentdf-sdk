"""Native installed Python API benchmark; fixture I/O and OAuth are untimed."""
import json
import sys
import time
import urllib.parse
import urllib.request
from pathlib import Path
from opentdf_tdf3 import Config, KASRoute, EncryptOptions, AccessToken, encrypt, decrypt

run = Path(sys.argv[1])
op, size, n = sys.argv[2], sys.argv[3], int(sys.argv[4])
raw = json.loads((run / 'private.json').read_text())
r = raw['Config']


def config():
    return Config(
        platform_url=r['PlatformURL'], kas_url=r['KASURL'],
        allowed_kas=[KASRoute(v['URL'], v['APIBaseURL']) for v in r['AllowedKAS']],
        allow_http=True, kas_public_key_pem=r['KASPublicKeyPEM'], kid=r['KID'],
        kas_algorithm='rsa:2048', session_algorithm='rsa:2048',
        auth_algorithm='ES256', token_provider_name='access-token',
    )


def acquire_token():
    # A 50 MiB batch outlasts the 300-second token TTL. Refresh in the host
    # before each timer; the SDK still receives a static provider during its call.
    form = urllib.parse.urlencode({
        'grant_type': 'client_credentials', 'client_id': 'opentdf-sdk',
        'client_secret': 'secret',
    }).encode()
    endpoint = 'http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token'
    with urllib.request.urlopen(urllib.request.Request(endpoint, data=form), timeout=15) as response:
        token = json.load(response)
    return token['access_token'], int(time.time()) + int(token['expires_in'])


input = (run / (size + '.input')).read_bytes()
archive = (run / (size + '.reference.tdf')).read_bytes() if op == 'decrypt' else None
samples = []
for i in range(-1, n):
    token, expires = acquire_token()
    start = time.perf_counter_ns()
    cfg = config()
    provider = lambda request: AccessToken(token, 'Bearer', expires)
    if op == 'encrypt':
        options = EncryptOptions(
            attributes=['https://example.com/attr/attr1/value/value1'],
            segment_size=2 << 20, has_segment_size=True, segment_hash_algorithm='GMAC',
        )
        output = encrypt(cfg, input, options, provider=provider).result(1800)
    else:
        output = decrypt(cfg, archive, provider=provider).result(1800).payload
    elapsed = (time.perf_counter_ns() - start) / 1e6
    if op == 'decrypt':
        assert output == input, 'plaintext mismatch'
    else:
        (run / f'python-{size}-{i}.tdf').write_bytes(output)
    if i >= 0:
        samples.append(elapsed)
print(json.dumps({'samples_ms': samples, 'correct': True, 'untimed_oauth_refreshes': n + 1}))
