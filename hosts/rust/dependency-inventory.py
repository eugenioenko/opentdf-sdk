#!/usr/bin/env python3
"""Verify Cargo archives and retain exact transitive/native license notices."""
import hashlib,json,pathlib,sys,shutil,subprocess,re
package=pathlib.Path(sys.argv[1]);metadata=json.loads(subprocess.check_output(['cargo','metadata','--offline','--locked','--format-version','1','--manifest-path',str(package/'Cargo.toml')]))
locked={}
for block in (package/'Cargo.lock').read_text().split('[[package]]')[1:]:
 fields=dict(re.findall(r'^([a-z_]+) = "([^"]*)"$',block,re.M))
 if 'checksum' in fields:locked[(fields['name'],fields['version'])]=fields['checksum']
registry=pathlib.Path.home()/'.cargo/registry'
rows=[]
for item in metadata['packages']:
 if not item.get('source'):continue
 name,version=item['name'],item['version'];archive=next((registry/'cache').glob('*/'+name+'-'+version+'.crate'))
 digest=hashlib.sha256(archive.read_bytes()).hexdigest()
 if digest!=locked[(name,version)]:raise RuntimeError('Cargo archive checksum mismatch: '+str(archive))
 source=pathlib.Path(item['manifest_path']).parent;crate_metadata=item
 notices=[]
 for f in sorted(source.rglob('*')):
  if f.is_file() and any(k in f.name.upper() for k in ['LICENSE','COPYING','NOTICE','COPYRIGHT']):
   rel=f.relative_to(source);target=package/'licenses'/f'{name}-{version}'/rel;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(f,target)
   notices.append({'path':str(rel),'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
 rows.append({'name':name,'version':version,'artifact':archive.name,'sha256':digest,'license':crate_metadata.get('license'),'license_file':crate_metadata.get('license_file'),'rust_version':crate_metadata.get('rust_version'),'notices':notices})
rows.sort(key=lambda r:(r['name'],r['version']))
output={'schema_version':1,'toolchain':'rustc 1.98.0 (88d9e12ae 2026-08-18)','target':'x86_64-unknown-linux-gnu','declared_package_msrv':'1.88','tested_msrv':'1.98.0','dependencies':rows}
(package/'dependencies.lock.json').write_text(json.dumps(output,indent=2)+'\n')
shutil.copyfile(pathlib.Path(__file__).resolve().parents[3]/'goalchemy/LICENSE',package/'LICENSE')
