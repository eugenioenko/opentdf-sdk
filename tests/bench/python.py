"""Native installed Python end-to-end public API benchmark."""
import json
import sys
import time
from pathlib import Path
from opentdf_tdf3 import Config, KASRoute, EncryptOptions, AccessToken, encrypt, decrypt

run = Path(sys.argv[1])
op, size, n = sys.argv[2], sys.argv[3], int(sys.argv[4])
assert op == 'e2e'
warmups = int(sys.argv[5]) if len(sys.argv) > 5 else 1
bulk_warmups = int(sys.argv[6]) if len(sys.argv) > 6 else 0
assert n > 0 and warmups > 0 and bulk_warmups >= 0
raw = json.loads((run / 'private.json').read_text())
r = raw['Config']
input = (run / (size + '.input')).read_bytes()
bulk_input = (run / '50.input').read_bytes() if bulk_warmups and size != '50' else input

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
samples, warmup_history, bulk_history = [], [], []
for i in range(-bulk_warmups - warmups, n):
    pair_input = bulk_input if i < -warmups else input
    start = time.perf_counter_ns()
    archive = encrypt(cfg, pair_input, options, provider=provider).result(1800)
    output = decrypt(cfg, archive, provider=provider).result(1800).payload
    elapsed = (time.perf_counter_ns() - start) / 1e6
    assert output == pair_input, 'plaintext mismatch'
    if i == -1 or i >= 0:
        (run / f'python-{size}-{i}.tdf').write_bytes(archive)
    if i >= 0:
        samples.append(elapsed)
    elif i < -warmups:
        bulk_history.append(elapsed)
    else:
        warmup_history.append(elapsed)
print(json.dumps({'samples_ms': samples, 'warmup_ms': warmup_history, 'bulk_warmup_ms': bulk_history, 'warmup_count': warmups, 'bulk_warmup_count': bulk_warmups, 'correct': True, 'kas_calls_expected': n + warmups + bulk_warmups}))
