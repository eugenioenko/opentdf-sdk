"""Native installed Python handle-API benchmark: one SDK per process, as the reference consumer."""
import json
import sys
import time
from pathlib import Path
from opentdf_tdf3 import SDK, AccessToken

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

token, expires = raw['Token'], int(raw['Expires'])
sdk = SDK(r['PlatformURL'], insecure_plaintext=True, token_endpoint=r['TokenURL'],
          token_provider=lambda request: AccessToken(token, 'Bearer', expires))
tdf_options = dict(
    disable_autoconfigure=True,
    kas_information=[{'url': r['KASURL'], 'public_key': r['KASPublicKeyPEM'], 'kid': r['KID'], 'algorithm': 'rsa:2048'}],
    wrapping_key_algorithm='rsa:2048', segment_size=2 << 20,
    attributes=['https://example.com/attr/attr1/value/value1'],
)
# load_tdf reuses the SDK's default RSA2048 session. Do not generate a replacement.
reader_options = dict(kas_allowlist=[r['KASURL']])
samples, warmup_history, bulk_history = [], [], []
for i in range(-bulk_warmups - warmups, n):
    pair_input = bulk_input if i < -warmups else input
    start = time.perf_counter_ns()
    archive = sdk.encrypt_bytes(pair_input, timeout=1800, **tdf_options)
    output = sdk.decrypt_bytes(archive, timeout=1800, **reader_options)
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
sdk.close()
print(json.dumps({'samples_ms': samples, 'warmup_ms': warmup_history, 'bulk_warmup_ms': bulk_history, 'warmup_count': warmups, 'bulk_warmup_count': bulk_warmups, 'correct': True, 'kas_calls_expected': n + warmups + bulk_warmups}))
