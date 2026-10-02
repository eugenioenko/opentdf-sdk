#!/usr/bin/env python3
"""Copy the pinned CLI; refresh only its source-built SDK tarball's lock identity."""
import base64
import hashlib
import json
import shutil
from pathlib import Path

sdk = Path(__file__).resolve().parents[1]
source = sdk.parent / 'web-sdk' / 'cli'
dest = sdk / '.local' / 'web-cli'
tar = sdk.parent / 'web-sdk' / 'lib' / 'opentdf-sdk-0.21.0.tgz'
dest.mkdir(parents=True, exist_ok=True)
for name in ('src', 'tests', 'bin'):
    shutil.copytree(source / name, dest / name, dirs_exist_ok=True)
shutil.copy2(source / 'tsconfig.json', dest)
pkg = json.loads((source / 'package.json').read_text())
lock = json.loads((source / 'package-lock.json').read_text())
uri = 'file:' + str(tar.resolve())
pkg['dependencies']['@opentdf/sdk'] = uri
lock['packages']['']['dependencies']['@opentdf/sdk'] = uri
node = lock['packages']['node_modules/@opentdf/sdk']
node['resolved'] = uri
node['integrity'] = 'sha512-' + base64.b64encode(hashlib.sha512(tar.read_bytes()).digest()).decode()
for name, value in [('package.json', pkg), ('package-lock.json', lock)]:
    (dest / name).write_text(json.dumps(value, indent=2) + '\n')
print('CLI local SDK tarball lock identity:', node['integrity'])
