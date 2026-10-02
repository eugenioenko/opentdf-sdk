#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOCAL="$SDK/.local"
RUN="$LOCAL/interop"
mkdir -p "$RUN"
"$SDK/scripts/platform-check.py"
GO="$LOCAL/bin/otdfctl"
WEB="$LOCAL/web-cli/bin/opentdf.mjs"
# These explicit flags select the Go CLI in-memory profile; no keyring or global profile is touched.
go_cli() { "$GO" "$@" --host http://localhost:8080 --with-client-creds-file "$RUN/client-creds.json"; }
web_cli() { node "$WEB" "$@" --platformUrl http://localhost:8080 --kasEndpoint http://localhost:8080/kas \
  --oidcEndpoint http://localhost:8888/auth/realms/opentdf --clientId opentdf-sdk --clientSecret secret --logLevel error; }
python3 - "$RUN" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1])
p.joinpath('client-creds.json').write_text('{"clientId":"opentdf-sdk","clientSecret":"secret"}')
p.joinpath('small.input').write_bytes(b'OpenTDF reference interoperability\n')
p.joinpath('binary.input').write_bytes(bytes(range(256))*16+b'\x00\xffTDF interop\n')
p.joinpath('empty.input').write_bytes(b'')
PY
# Save actual help for the tested revisions.
"$GO" encrypt --help > "$RUN/go-encrypt-help.txt"
"$GO" decrypt --help > "$RUN/go-decrypt-help.txt"
node "$WEB" encrypt --help > "$RUN/web-encrypt-help.txt"
node "$WEB" decrypt --help > "$RUN/web-decrypt-help.txt"
allowed=https://example.com/attr/attr1/value/value1
denied=https://example.com/attr/attr1/value/value2
for name in small binary empty; do
  rm -f "$RUN/$name.go.tdf" "$RUN/$name.web.tdf" "$RUN/$name.go-to-web" "$RUN/$name.web-to-go"
  go_cli encrypt "$RUN/$name.input" --attr "$allowed" --wrapping-key-algorithm rsa:2048 \
    --out "$RUN/$name.go.tdf" > "$RUN/$name.go-encrypt.log" 2>&1
  web_cli decrypt "$RUN/$name.go.tdf" --rewrapKeyType rsa:2048 --allowList http://localhost:8080 \
    --output "$RUN/$name.go-to-web" > "$RUN/$name.web-decrypt.log" 2>&1
  cmp "$RUN/$name.input" "$RUN/$name.go-to-web"
  web_cli encrypt "$RUN/$name.input" --attributes "$allowed" --encapKeyType rsa:2048 \
    --output "$RUN/$name.web.tdf" > "$RUN/$name.web-encrypt.log" 2>&1
  go_cli decrypt "$RUN/$name.web.tdf" --session-key-algorithm rsa:2048 --kas-allowlist http://localhost:8080/kas \
    --out "$RUN/$name.web-to-go" > "$RUN/$name.go-decrypt.log" 2>&1
  cmp "$RUN/$name.input" "$RUN/$name.web-to-go"
  echo "PASS $name: Go -> TypeScript and TypeScript -> Go plaintext bytes"
done
# Both SDKs must observe a real PDP denial for the same registered but unentitled attribute.
go_cli encrypt "$RUN/binary.input" --attr "$denied" --out "$RUN/denied.go.tdf" > "$RUN/denied.go-encrypt.log" 2>&1
web_cli encrypt "$RUN/binary.input" --attributes "$denied" --output "$RUN/denied.web.tdf" > "$RUN/denied.web-encrypt.log" 2>&1
rm -f "$RUN/denied.go-to-web" "$RUN/denied.web-to-go"
if web_cli decrypt "$RUN/denied.go.tdf" --allowList http://localhost:8080 --output "$RUN/denied.go-to-web" \
    > "$RUN/denied.web-decrypt.log" 2>&1; then echo 'ERROR: TypeScript accepted denied policy' >&2; exit 1; fi
if go_cli decrypt "$RUN/denied.web.tdf" --kas-allowlist http://localhost:8080/kas --out "$RUN/denied.web-to-go" \
    > "$RUN/denied.go-decrypt.log" 2>&1; then echo 'ERROR: Go accepted denied policy' >&2; exit 1; fi
rg -qi 'permission.denied|pdp.denied|forbidden' "$RUN/denied.web-decrypt.log"
rg -qi 'permission.denied|pdp.denied|forbidden' "$RUN/denied.go-decrypt.log"
[[ ! -s "$RUN/denied.go-to-web" && ! -s "$RUN/denied.web-to-go" ]]
python3 - "$RUN" <<'PY'
from pathlib import Path
import hashlib, json, sys, zipfile
p=Path(sys.argv[1]); report={'pairs':[], 'denied':{'Go':True,'TypeScript':True},'auth':'client-credentials Bearer','wrapping':'rsa:2048','session':'rsa:2048','adapter':None}
for name in ('small','binary','empty'):
    raw=p.joinpath(name+'.input').read_bytes()
    for producer,consumer in [('go','web'),('web','go')]:
        with zipfile.ZipFile(p/f'{name}.{producer}.tdf') as z:
            manifest=json.loads(z.read('0.manifest.json'))
            report['pairs'].append({'payload':name,'producer':producer,'consumer':consumer,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'entries':z.namelist(),'manifest_version':manifest.get('schemaVersion'),'key_access':manifest['encryptionInformation']['keyAccess']})
p.joinpath('results.json').write_text(json.dumps(report,indent=2)+'\n')
PY
echo 'PASS real KAS allow/deny; results: .local/interop/results.json'
