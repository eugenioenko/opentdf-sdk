#!/usr/bin/env bash
# Fast shared-source format/protocol checks; does not substitute for real KAS.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
OUT=${TDF_DELIVERY_OFFLINE_OUT:-"$SDK/.local/phase7/offline"}
mkdir -p "$OUT"
export GOTOOLCHAIN=go1.25.14
cd "$SDK"
timeout 180 go test -json ./tdf/... > "$OUT/format.jsonl"
timeout 180 go test -json -run '^(TestStrictResponseAndTamper|TestExactBoundPolicyAndSID|TestDiscoveryAndOAuthValidationBeforeResourceAccess|TestAllowlistBeforeCredentials)$' . > "$OUT/protocol.jsonl"
python3 - "$SDK" "$OUT" <<'PY'
import hashlib,json,pathlib,sys
sdk,out=map(pathlib.Path,sys.argv[1:]);tests=[]
for log in ('format.jsonl','protocol.jsonl'):
 for line in (out/log).read_text().splitlines():
  event=json.loads(line)
  if event['Action']=='skip':raise RuntimeError('required offline check skipped '+str(event))
  if event['Action']=='pass' and 'Test' in event:tests.append({'package':event['Package'],'test':event['Test']})
sources={str(p.relative_to(sdk)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(sdk.rglob('*.go')) if '.local' not in p.parts and 'out' not in p.parts}
(out/'receipt.json').write_text(json.dumps({'status':0,'scope':'focused shared-source format/rejection gates; positive KAS and native boundary propagation are separate','tests':tests,'source_hashes':sources,'logs':{n:hashlib.sha256((out/n).read_bytes()).hexdigest() for n in ('format.jsonl','protocol.jsonl')}},indent=2)+'\n')
print('PASS offline format/protocol',len(tests),'named checks')
PY
