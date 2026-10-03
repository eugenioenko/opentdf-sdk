#!/usr/bin/env python3
"""Create an isolated source checkout and reviewable Compose config, without services."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess

SDK = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace', type=Path, required=True)
    parser.add_argument('--project', required=True)
    args = parser.parse_args()
    if not args.project.startswith('phase7-') or any(c not in 'abcdefghijklmnopqrstuvwxyz0123456789-' for c in args.project):
        raise RuntimeError('project must be a unique phase7- name')
    workspace = args.workspace.resolve()
    workspace.mkdir(parents=True, exist_ok=False)
    revisions = {}
    for name in ('sdk','goalchemy','platform','web-sdk'):
        source = SDK if name == 'sdk' else SDK.parent/name
        revision = subprocess.check_output(['git','-C',str(source),'rev-parse','HEAD'], text=True).strip()
        subprocess.run(['git','clone','--quiet','--no-hardlinks','--no-checkout',str(source),str(workspace/name)], check=True)
        subprocess.run(['git','-C',str(workspace/name),'checkout','--quiet','--detach',revision], check=True)
        revisions[name] = revision
    # Include exactly tracked SDK files and new delivery-owned source files.
    paths = subprocess.check_output(['git','-C',str(SDK),'ls-files','--cached','--others','--exclude-standard','-z']).decode().split('\0')
    manifest = {}
    for name in filter(None, paths):
        source = SDK/name
        if source.is_file():
            output = workspace/'sdk'/name
            output.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source,output)
            manifest[name] = hashlib.sha256(source.read_bytes()).hexdigest()
    # Shared downloads/toolchains are pinned; consumers, fixtures, outputs and
    # services live exclusively in this fresh checkout's ignored private tree.
    (workspace/'goalchemy/.toolchains').symlink_to(SDK.parent/'goalchemy/.toolchains',target_is_directory=True)
    environment = os.environ.copy()
    image = args.project+'-platform:local'
    environment.update(TDF_COMPOSE_PROJECT=args.project,TDF_PLATFORM_IMAGE=image)
    config = subprocess.check_output(['docker','compose','--project-name',args.project,'--env-file',str(workspace/'sdk/dev/images.env'),'-f',str(workspace/'sdk/dev/compose.yaml'),'config'],env=environment)
    (workspace/'compose-rendered.yaml').write_bytes(config)
    receipt = {'project':args.project,'image':image,'workspace':str(workspace),'revisions':revisions,'sdk_source_sha256':manifest,
               'compose_configuration_sha256':hashlib.sha256(config).hexdigest(),
               'bind_mount_root':str(workspace/'sdk/.local'),'ports':[5432,8888,9000,8080],
               'start_command':['timeout','1800',str(workspace/'sdk/scripts/platform.sh'),'up'],
               'cleanup_command':['timeout','90','docker','compose','--project-name',args.project,'--env-file',str(workspace/'sdk/dev/images.env'),'-f',str(workspace/'sdk/dev/compose.yaml'),'down','--remove-orphans'],
               'service_mutations':False}
    (workspace/'prepare-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(workspace/'prepare-receipt.json')


if __name__ == '__main__':
    main()
