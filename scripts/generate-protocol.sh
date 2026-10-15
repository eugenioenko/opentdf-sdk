#!/usr/bin/env bash
# Generates the protocol code from the platform's .proto files (../platform/service
# at the revision pinned in references.lock.json):
#   src/protocol                    Goalchemy-subset messages and Connect clients for
#                                   what the shared source reads (protoc-gen-goalchemy)
#   src/hosts/typescript/protocol   protobuf-es messages and services (protoc-gen-es)
#   src/hosts/python/protocol       protobuf Python messages (protoc)
#   src/hosts/java/protocol         protobuf-java messages (protoc)
#   src/hosts/csharp/protocol       Google.Protobuf messages (protoc)
#   src/hosts/rust/protocol         prost messages with pbjson serde (protoc-gen-prost,
#                                   protoc-gen-prost-serde)
#   src/hosts/swift/protocol        SwiftProtobuf messages (protoc-gen-swift)
#   src/hosts/*/services.*          the hosts' service accessors, from the reference
#                                   sdkconnect (scripts/generate-service-clients.py)
# --check regenerates into a temporary directory and fails when any output differs.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
PLATFORM=${PLATFORM_ROOT:-"$SDK/../platform"}
TOOLS=${PROTOCOL_TOOLS_DIR:-"$SDK/.local/protocol-tools"}
BUF_VERSION=v1.70.0
PROTOC_VERSION=33.6
PROTOC_SHA256=9c49962391ff8b5754342509efc3885db91dfe11a358e1e98c9a71518f0ad199
PROTOC_GEN_PROST_VERSION=0.5.0
PROTOC_GEN_PROST_SERDE_VERSION=0.4.0
SWIFT_PROTOBUF_VERSION=1.38.1
SWIFT_PROTOBUF_COMMIT=55d7a1cc5666b85c13464aea1c4b4a90feccb4c8
export GOTOOLCHAIN=go1.25.14

# The services behind the reference's sdkconnect clients, plus kas.AccessService
# for the shared source.
SERVICES=(
  authorization/authorization.proto
  authorization/v2/authorization.proto
  entityresolution/entity_resolution.proto
  entityresolution/v2/entity_resolution.proto
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
# The services' imports other than buf/validate, which the Rust and Swift
# messages do not reference. Java and C# descriptors do, so those generators
# include every import.
IMPORTS=(common/common.proto entity/entity.proto policy/objects.proto policy/selectors.proto)
SHARED=(common/common.proto entity/entity.proto policy/objects.proto policy/selectors.proto kas/kas.proto
  wellknownconfiguration/wellknown_configuration.proto policy/kasregistry/key_access_server_registry.proto
  policy/attributes/attributes.proto authorization/authorization.proto)

# Only what the shared source reads; every other method is called through
# SDK.Call with JSON and typed by the host's own protobuf messages.
INCLUDE=(
  kas.AccessService.Rewrap
  kas.AccessService.PublicKey
  kas.UnsignedRewrapRequest
  wellknownconfiguration.WellKnownService.GetWellKnownConfiguration
  policy.kasregistry.KeyAccessServerRegistryService.ListKeyAccessServers
  policy.attributes.AttributesService.ListAttributes
  policy.attributes.AttributesService.GetAttribute
  policy.attributes.AttributesService.GetAttributeValuesByFqns
  authorization.AuthorizationService.GetEntitlements
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
if ! cmp -s "$SDK/scripts/protocol-tools/package-lock.json" "$TOOLS/npm/package-lock.json"; then
  rm -rf "$TOOLS/npm"
  mkdir -p "$TOOLS/npm"
  cp "$SDK/scripts/protocol-tools/package.json" "$SDK/scripts/protocol-tools/package-lock.json" "$TOOLS/npm/"
  (cd "$TOOLS/npm"; npm ci --no-audit --no-fund --ignore-scripts >/dev/null)
fi
if [[ ! -x "$TOOLS/protoc-$PROTOC_VERSION/bin/protoc" ]]; then
  archive="$TOOLS/protoc-$PROTOC_VERSION-linux-x86_64.zip"
  curl -fsSL -o "$archive" "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip"
  echo "$PROTOC_SHA256  $archive" | sha256sum -c --quiet
  rm -rf "$TOOLS/protoc-$PROTOC_VERSION"
  python3 -c 'import sys,zipfile; zipfile.ZipFile(sys.argv[1]).extractall(sys.argv[2])' "$archive" "$TOOLS/protoc-$PROTOC_VERSION"
  chmod +x "$TOOLS/protoc-$PROTOC_VERSION/bin/protoc"
fi
cargo install --quiet --locked --root "$TOOLS/cargo" "protoc-gen-prost@$PROTOC_GEN_PROST_VERSION" "protoc-gen-prost-serde@$PROTOC_GEN_PROST_SERDE_VERSION"
SWIFT_PLUGIN="$TOOLS/swift-protobuf-$SWIFT_PROTOBUF_VERSION/.build/release/protoc-gen-swift"
if [[ ! -x "$SWIFT_PLUGIN" ]]; then
  rm -rf "$TOOLS/swift-protobuf-$SWIFT_PROTOBUF_VERSION"
  git clone --quiet --depth 1 --branch "$SWIFT_PROTOBUF_VERSION" https://github.com/apple/swift-protobuf.git "$TOOLS/swift-protobuf-$SWIFT_PROTOBUF_VERSION"
  if [[ "$(git -C "$TOOLS/swift-protobuf-$SWIFT_PROTOBUF_VERSION" rev-parse HEAD)" != "$SWIFT_PROTOBUF_COMMIT" ]]; then
    echo "swift-protobuf $SWIFT_PROTOBUF_VERSION is not $SWIFT_PROTOBUF_COMMIT" >&2
    exit 1
  fi
  SWIFT=swift
  if [[ -x "$ROOT/.toolchains/swift-6.4.0/usr/bin/swift" ]]; then SWIFT="$ROOT/.toolchains/swift-6.4.0/usr/bin/swift"; fi
  "$SWIFT" build --package-path "$TOOLS/swift-protobuf-$SWIFT_PROTOBUF_VERSION" -c release --product protoc-gen-swift
fi

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cat > "$WORK/goalchemy.yaml" <<EOF
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
    out: $WORK/goalchemy
    strategy: all
    opt:
      - go_import_prefix=opentdf-local/sdk/src/protocol
      - include=$(IFS=';'; echo "${INCLUDE[*]}")
EOF
cat > "$WORK/hosts.yaml" <<EOF
version: v2
plugins:
  - local: $TOOLS/npm/node_modules/.bin/protoc-gen-es
    out: $WORK/typescript
    include_imports: true
    opt:
      - target=ts
      - import_extension=.js
  - protoc_builtin: python
    protoc_path: $TOOLS/protoc-$PROTOC_VERSION/bin/protoc
    out: $WORK/python
    include_imports: true
EOF
# Java and C# messages live under the SDK's own package and namespace, with
# buf/validate among them, as the Python and TypeScript ones do.
cat > "$WORK/java.yaml" <<EOF
version: v2
managed:
  enabled: true
  override:
    - file_option: java_package_prefix
      value: io.opentdf.tdf3.protocol
    - file_option: optimize_for
      value: CODE_SIZE
plugins:
  - protoc_builtin: java
    protoc_path: $TOOLS/protoc-$PROTOC_VERSION/bin/protoc
    out: $WORK/java
    include_imports: true
EOF
cat > "$WORK/csharp.yaml" <<EOF
version: v2
managed:
  enabled: true
  override:
    - file_option: csharp_namespace_prefix
      value: OpenTDF.Protocol
plugins:
  - protoc_builtin: csharp
    protoc_path: $TOOLS/protoc-$PROTOC_VERSION/bin/protoc
    out: $WORK/csharp
    include_imports: true
    opt:
      - base_namespace=OpenTDF.Protocol
EOF
cat > "$WORK/native.yaml" <<EOF
version: v2
plugins:
  - local: $SWIFT_PLUGIN
    out: $WORK/swift
    opt:
      - Visibility=Public
      - FileNaming=PathToUnderscores
  - local: $TOOLS/cargo/bin/protoc-gen-prost
    out: $WORK/rust
    strategy: all
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
      - flat_output_dir
  - local: $TOOLS/cargo/bin/protoc-gen-prost-serde
    out: $WORK/rust
    strategy: all
    opt:
      - extern_path=.google.protobuf=::pbjson_types
      - flat_output_dir
      - ignore_unknown_fields
EOF
paths() { for f in "$@"; do printf -- '--path\0service/%s\0' "$f"; done; }
(cd "$PLATFORM"; paths "${SHARED[@]}" | xargs -0 "$BUF" generate --template "$WORK/goalchemy.yaml")
(cd "$PLATFORM"; paths "${SERVICES[@]}" | xargs -0 "$BUF" generate --template "$WORK/hosts.yaml")
(cd "$PLATFORM"; paths "${SERVICES[@]}" | xargs -0 "$BUF" generate --template "$WORK/java.yaml")
(cd "$PLATFORM"; paths "${SERVICES[@]}" | xargs -0 "$BUF" generate --template "$WORK/csharp.yaml")
(cd "$PLATFORM"; paths "${SERVICES[@]}" "${IMPORTS[@]}" | xargs -0 "$BUF" generate --template "$WORK/native.yaml")

# prost writes one file per package; the crate's protocol module nests them as
# the packages nest, which the messages' super:: paths expect.
python3 - "$WORK/rust" <<'PY'
import pathlib,sys
root=pathlib.Path(sys.argv[1])
tree={}
for f in sorted(root.glob('*.rs')):
    if not f.name.endswith('.serde.rs'):
        node=tree
        for part in f.name[:-3].split('.'):
            node=node.setdefault(part,{})
        node['']=f.name
def emit(node,depth):
    out=[]
    if '' in node:
        out.append('    '*depth+'include!("'+node['']+'");')
    for name in sorted(k for k in node if k):
        ident='r#'+name if name in ('unsafe','type','mod','use') else name
        out+=['    '*depth+'pub mod '+ident+' {',*emit(node[name],depth+1),'    '*depth+'}']
    return out
(root/'mod.rs').write_text('// Code generated by scripts/generate-protocol.sh from the prost output. DO NOT EDIT.\n'+'\n'.join(emit(tree,0))+'\n')
PY

# protoc's Python imports are absolute from the proto root; the package ships
# them under opentdf_tdf3.protocol.
python3 - "$WORK/python" <<'PY'
import pathlib,re,sys
root=pathlib.Path(sys.argv[1])
for d in [root,*sorted(p for p in root.rglob('*') if p.is_dir())]:
    (d/'__init__.py').touch()
def local(match):
    module=match[1]
    if (root/module.replace('.','/')).is_dir():
        return 'from opentdf_tdf3.protocol.'+module+' import'
    return match[0]
for f in sorted(root.rglob('*_pb2.py')):
    f.write_text(re.sub(r'^from ([\w.]+) import', local, f.read_text(), flags=re.M))
PY
python3 "$SDK/scripts/generate-service-clients.py" "$PLATFORM" "$WORK/services"

OUTPUTS=(
  "goalchemy:src/protocol"
  "typescript:src/hosts/typescript/protocol"
  "python:src/hosts/python/protocol"
  "services/go/services.go.in:src/hosts/go/services.go.in"
  "services/typescript/services.ts.in:src/hosts/typescript/services.ts.in"
  "services/python/services.py.in:src/hosts/python/services.py.in"
  "java:src/hosts/java/protocol"
  "csharp:src/hosts/csharp/protocol"
  "rust:src/hosts/rust/protocol"
  "swift:src/hosts/swift/protocol"
  "services/java/PlatformServices.java.in:src/hosts/java/PlatformServices.java.in"
  "services/csharp/Services.cs.in:src/hosts/csharp/Services.cs.in"
  "services/rust/services.rs.in:src/hosts/rust/services.rs.in"
  "services/swift/Services.swift.in:src/hosts/swift/Services.swift.in"
  "services/c/opentdf_services.h:src/hosts/c/opentdf_services.h"
  "services/c/opentdf_services.c:src/hosts/c/opentdf_services.c"
)
if [[ "${1:-}" == --check ]]; then
  status=0
  for output in "${OUTPUTS[@]}"; do
    if ! diff -r "$WORK/${output%%:*}" "$SDK/${output#*:}"; then
      status=1
    fi
  done
  if [[ $status != 0 ]]; then
    echo "generated protocol code is not current; run scripts/generate-protocol.sh" >&2
    exit 1
  fi
  echo "generated protocol code is current"
  exit 0
fi
for output in "${OUTPUTS[@]}"; do
  rm -rf "${SDK:?}/${output#*:}"
  cp -r "$WORK/${output%%:*}" "$SDK/${output#*:}"
done
