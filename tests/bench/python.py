"""Native installed Python end-to-end public API benchmark."""
import json
import sys
import time
from pathlib import Path
from opentdf_tdf3 import Config, KASRoute, EncryptOptions, AccessToken, encrypt, decrypt

run = Path(sys.argv[1])
op, size, n = sys.argv[2], sys.argv[3], int(sys.argv[4])
assert op == 'e2e'
raw = json.loads((run / 'private.json').read_text())
r = raw['Config']
input = (run / (size + '.input')).read_bytes()
cfg = Config(
    platform_url=r['PlatformURL'], kas_url=r['KASURL'],
    allowed_kas=[KASRoute(v['URL'], v['APIBaseURL']) for v in r['AllowedKAS']],
    allow_http=True, kas_public_key_pem=r['KASPublicKeyPEM'], kid=r['KID'],
    kas_algorithm='rsa:2048', session_algorithm='rsa:2048',
    auth_algorithm='ES256', token_provider_name='access-token',
)
provider = lambda request: AccessToken(raw['Token'], 'Bearer', raw['Expires'])
options = EncryptOptions(
    attributes=['https://example.com/attr/attr1/value/value1'],
    segment_size=2 << 20, has_segment_size=True, segment_hash_algorithm='GMAC',
)
samples = []
for i in range(-1, n):
    start = time.perf_counter_ns()
    archive = encrypt(cfg, input, options, provider=provider).result(1800)
    output = decrypt(cfg, archive, provider=provider).result(1800).payload
    elapsed = (time.perf_counter_ns() - start) / 1e6
    assert output == input, 'plaintext mismatch'
    (run / f'python-{size}-{i}.tdf').write_bytes(archive)
    if i >= 0:
        samples.append(elapsed)
print(json.dumps({'samples_ms': samples, 'correct': True, 'kas_calls_expected': n + 1}))
