#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
PLATFORM="$SDK/../platform"
LOCAL="$SDK/.local"
mkdir -p "$LOCAL/bin" "$LOCAL/logs" "$LOCAL/keys"
# CLI smoke uses explicit --host/auth flags, selecting the reference CLI in-memory profile.
export npm_config_cache="$LOCAL/npm-cache"
compose() { docker compose --env-file "$SDK/dev/images.env" -f "$SDK/dev/compose.yaml" "$@"; }
check_refs() {
  python3 - "$SDK" <<'PY'
import json, subprocess, sys
from pathlib import Path
root = Path(sys.argv[1])
lock = json.loads((root / 'references.lock.json').read_text())
for name in ('platform', 'web-sdk'):
    repo = lock['repositories'][name]
    path = root / repo['path']
    actual = subprocess.check_output(['git', '-C', str(path), 'rev-parse', 'HEAD'], text=True).strip()
    if actual != repo['revision']:
        raise SystemExit(f'{name}: revision mismatch ({actual}); restore references.lock.json revision')
    if subprocess.check_output(['git', '-C', str(path), 'status', '--porcelain', '--untracked-files=no'], text=True):
        raise SystemExit(f'{name}: tracked reference changes; refusing to build')
PY
}
wait_url() {
  local url=$1
  for ((i=0; i<120; i++)); do
    if curl --connect-timeout 2 --max-time 5 -fsS "$url" >/dev/null 2>&1; then return; fi
    sleep 2
  done
  echo "Readiness timed out: $url; use scripts/platform.sh logs" >&2
  return 1
}
init() {
  check_refs
  if [[ ! -s "$LOCAL/keys/kas-private.pem" ]]; then
    (umask 077; openssl req -x509 -newkey rsa:2048 -nodes -keyout "$LOCAL/keys/kas-private.pem" \
      -out "$LOCAL/keys/kas-cert.pem" -subj '/CN=tdf-sdk-local-kas' -days 3650 > "$LOCAL/logs/keygen.log" 2>&1)
  fi
  # Initialize only this bind mount for the pinned image UID/GID, independent of host UID.
  mkdir -p "$LOCAL/keycloak"
  docker run --rm --user 0 -v "$LOCAL/keycloak:/data" \
    "$(sed -n 's/^RUNTIME_IMAGE=//p' "$SDK/dev/images.env")" \
    sh -c 'chown -R 1000:1000 /data && chmod 0700 /data'
  printf '*\n!bin/\n!bin/opentdf\n' > "$LOCAL/.dockerignore"
}
build() {
  check_refs
  # Use explicit toolchain; all build/download caches stay in ignored workspace storage.
  export GOTOOLCHAIN=go1.25.14 GOCACHE="$LOCAL/go-build-cache" GOMODCACHE="$LOCAL/go-mod-cache"
  (cd "$PLATFORM"; go build -o "$LOCAL/bin/opentdf" ./service)
  (cd "$PLATFORM"; go build -ldflags '-X github.com/opentdf/platform/otdfctl/pkg/config.TestMode=true' -o "$LOCAL/bin/otdfctl" ./otdfctl)
  compose build platform
  (cd "$SDK/../web-sdk/lib"; npm ci; npm pack)
  "$SDK/scripts/build-web-cli.py"
  (cd "$LOCAL/web-cli"; npm ci; npm run build)
  check_refs
  python3 - "$SDK" <<'PY'
import json, sys
from pathlib import Path
p=Path(sys.argv[1]); (p/'.local/build-references.json').write_text((p/'references.lock.json').read_text())
PY
}
provision() {
  for ((i=0; i<120; i++)); do
    if compose exec -T postgres pg_isready -U postgres -d opentdf >/dev/null 2>&1; then break; fi
    sleep 2
  done
  compose exec -T postgres pg_isready -U postgres -d opentdf >/dev/null
  wait_url http://localhost:8888/auth/realms/master/.well-known/openid-configuration
  "$LOCAL/bin/opentdf" provision keycloak --file "$PLATFORM/service/cmd/keycloak_data.yaml" \
    > "$LOCAL/logs/keycloak-provision.log" 2>&1
  # Fixture provision uses a source-relative path; mount the pinned source fixture directory.
  compose run --rm --no-deps -w /source -v "$PLATFORM/service/internal/fixtures:/source/service/internal/fixtures:ro" \
    platform provision fixtures --config-file=/config/opentdf.yaml > "$LOCAL/logs/fixtures-provision.log" 2>&1
  compose exec -T postgres psql -U postgres -d opentdf -v ON_ERROR_STOP=1 \
    < "$SDK/dev/local-kas.sql" > "$LOCAL/logs/local-kas-provision.log" 2>&1
}
up() {
  init
  if [[ ! -s "$LOCAL/build-references.json" ]] || ! cmp -s "$SDK/references.lock.json" "$LOCAL/build-references.json"; then build; fi
  compose up -d postgres keycloak
  # Do not reprovision an initialized database; provisioning is also an explicit operator command.
  if [[ ! -s "$LOCAL/provisioned" ]]; then
    provision
    date -u +%FT%TZ > "$LOCAL/provisioned"
  fi
  compose up -d platform
  wait_url http://localhost:8080/healthz
  "$SDK/scripts/platform-check.py"
}
case "${1:-help}" in
  init) init ;;
  build) init; build ;;
  provision) provision ;;
  up|start) up ;;
  down|stop) compose down ;;
  status) compose ps --all ;;
  logs) compose logs --tail=100 "${@:2}" ;;
  ready|check) wait_url http://localhost:8080/healthz; "$SDK/scripts/platform-check.py" ;;
  smoke) "$SDK/scripts/reference-smoke.sh" ;;
  *) echo 'Usage: scripts/platform.sh {init|build|up|provision|ready|check|status|logs|down|smoke}'; exit 2 ;;
esac
