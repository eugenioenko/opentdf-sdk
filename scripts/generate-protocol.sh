#!/usr/bin/env bash
# Generates src/protocol from the platform's .proto files (../platform/service at
# the revision pinned in references.lock.json) with protoc-gen-goalchemy from the
# pinned Goalchemy checkout and buf. --check regenerates into a temporary
# directory and fails when src/protocol differs.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
PLATFORM=${PLATFORM_ROOT:-"$SDK/../platform"}
TOOLS=${PROTOCOL_TOOLS_DIR:-"$SDK/.local/protocol-tools"}
BUF_VERSION=v1.70.0
export GOTOOLCHAIN=go1.25.14

FILES=(
  common/common.proto
  entity/entity.proto
  policy/objects.proto
  policy/selectors.proto
  authorization/authorization.proto
  authorization/v2/authorization.proto
  entityresolution/entity_resolution.proto
  entityresolution/v2/entity_resolution.proto
  kas/kas.proto
  policy/actions/actions.proto
  policy/attributes/attributes.proto
  policy/dynamicvaluemapping/dynamic_value_mapping.proto
  policy/kasregistry/key_access_server_registry.proto
  policy/keymanagement/key_management.proto
  policy/namespaces/namespaces.proto
  policy/obligations/obligations.proto
  policy/registeredresources/registered_resources.proto
  policy/resourcemapping/resource_mapping.proto
  policy/subjectmapping/subject_mapping.proto
  policy/unsafe/unsafe.proto
  wellknownconfiguration/wellknown_configuration.proto
)

lock() {
  python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["repositories"][sys.argv[2]]["revision"])' "$SDK/references.lock.json" "$1"
}
for repo in platform:"$PLATFORM" goalchemy:"$ROOT"; do
  name=${repo%%:*} dir=${repo#*:}
  want=$(lock "$name")
  have=$(git -C "$dir" rev-parse HEAD)
  if [[ "$have" != "$want" ]]; then
    echo "$dir is at $have; references.lock.json pins $name $want" >&2
    exit 1
  fi
done

mkdir -p "$TOOLS"
TOOLS=$(cd "$TOOLS" && pwd)
(cd "$ROOT"; go build -o "$TOOLS/protoc-gen-goalchemy" ./cmd/protoc-gen-goalchemy)
BUF=${BUF:-"$TOOLS/buf"}
if [[ ! -x "$BUF" ]] || [[ "$("$BUF" --version)" != "${BUF_VERSION#v}" ]]; then
  GOBIN="$TOOLS" go install "github.com/bufbuild/buf/cmd/buf@$BUF_VERSION"
  BUF="$TOOLS/buf"
fi

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cat > "$WORK/buf.gen.yaml" <<EOF
version: v2
managed:
  enabled: true
  disable:
    - file_option: go_package
      module: buf.build/bufbuild/protovalidate
  override:
    - file_option: go_package_prefix
      value: github.com/opentdf/platform/protocol/go
plugins:
  - local: $TOOLS/protoc-gen-goalchemy
    out: $WORK/out
    opt:
      - go_import_prefix=opentdf-local/sdk/src/protocol
EOF
paths=()
for f in "${FILES[@]}"; do paths+=(--path "service/$f"); done
(cd "$PLATFORM"; "$BUF" generate --template "$WORK/buf.gen.yaml" "${paths[@]}")

if [[ "${1:-}" == --check ]]; then
  if ! diff -r "$WORK/out" "$SDK/src/protocol"; then
    echo "src/protocol is not current; run scripts/generate-protocol.sh" >&2
    exit 1
  fi
  echo "src/protocol is current"
  exit 0
fi
rm -rf "$SDK/src/protocol"
cp -r "$WORK/out" "$SDK/src/protocol"
