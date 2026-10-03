#!/usr/bin/env bash
set -euo pipefail
umask 077
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
LOCAL="$SDK/.local"
PROFILE=${1:-help}
compose() { timeout 90 docker compose --project-name "${TDF_COMPOSE_PROJECT:-tdf-sdk}" --env-file "$SDK/dev/images.env" -f "$SDK/dev/compose.yaml" "$@"; }
secure_compose() { timeout 90 docker compose --project-name "${TDF_COMPOSE_PROJECT:-tdf-sdk}" --env-file "$SDK/dev/images.env" -f "$SDK/dev/compose.yaml" -f "$SDK/dev/compose.profiles.yaml" "$@"; }
run() { (cd "$SDK/tests/interop/profiles"; GOTOOLCHAIN=go1.25.14 GOCACHE="$LOCAL/go-build-cache" GOMODCACHE="$LOCAL/go-mod-cache" timeout 180 go run . "$@"); }
case "$PROFILE" in
  ec|dpop|basic) ;;
  check|smoke)
    run pins
    [[ -s "$LOCAL/profiles/active" ]] || { echo 'No verified selected profile; use platform-profile.sh basic, ec or dpop' >&2; exit 1; }
    selected=$(cat "$LOCAL/profiles/active")
    case "$selected" in
      basic)
        [[ "$PROFILE" == check ]] || { echo 'Use make interop-smoke for the basic profile' >&2; exit 1; }
        "$SDK/scripts/platform-check.py" ;;
      ec|dpop) run "$PROFILE" ;;
      *) echo 'Invalid profile marker; select basic, ec or dpop explicitly' >&2; exit 1 ;;
    esac
    exit ;;
  *) echo 'Usage: scripts/platform-profile.sh {ec|dpop|basic|check|smoke}'; exit 2 ;;
esac
# The runner verifies source pins and clean tracked references before any mutation.
run pins
python3 - "$SDK" <<'CHECK'
import json, sys
from pathlib import Path
root = Path(sys.argv[1])
try:
    lock = json.loads((root / 'references.lock.json').read_text())
    built = json.loads((root / '.local/build-references.json').read_text())
    assert all(lock['repositories'][name]['revision'] == built['repositories'][name]['revision']
               for name in ('platform', 'web-sdk'))
except (OSError, ValueError, KeyError, AssertionError):
    raise SystemExit('Pinned platform/Web artifacts missing/stale: run make platform-build')
CHECK
[[ -s "$LOCAL/provisioned" && -s "$LOCAL/bin/opentdf" ]] || { echo 'Run make platform-up first' >&2; exit 1; }
mkdir -p "$LOCAL/profiles"
# Stop only our platform while modifying its registry; preserve existing r1 key and fixtures.
rm -f "$LOCAL/profiles/active"
trap 'echo "Profile switch failed; recover with ./scripts/platform-profile.sh basic. Details remain under ignored .local/profiles/." >&2' ERR
compose stop platform
compose exec -T postgres psql -U postgres -d opentdf -v ON_ERROR_STOP=1 < "$SDK/dev/secure-profile.sql" > "$LOCAL/profiles/registry-reset.log" 2>&1
if [[ "$PROFILE" == basic ]]; then
  compose up -d --no-deps --force-recreate platform
else
  run provision "$PROFILE"
  compose exec -T postgres psql -U postgres -d opentdf -v ON_ERROR_STOP=1 < "$LOCAL/profiles/registry.sql" > "$LOCAL/profiles/registry-provision.log" 2>&1
  secure_compose up -d --no-deps --force-recreate platform
fi
for ((i=0; i<60; i++)); do
  if curl --connect-timeout 2 --max-time 3 -fsS http://localhost:8080/healthz >/dev/null 2>&1; then break; fi
  sleep 1
done
curl --connect-timeout 2 --max-time 3 -fsS http://localhost:8080/healthz >/dev/null
if [[ "$PROFILE" == basic ]]; then "$SDK/scripts/platform-check.py"; else run check "$PROFILE"; fi
printf '%s\n' "$PROFILE" > "$LOCAL/profiles/active"
