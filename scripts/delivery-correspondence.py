#!/usr/bin/env python3
"""Read-only local correspondence to retained accepted matrices; never replay them.

Historical trees are review evidence in ignored storage, not CI dependencies.
CI generates fresh job evidence through delivery-job.py instead.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import itertools
import json
import re

SDK = Path(__file__).resolve().parents[1]
TREES = {
    'go':'goalchemy/out/typescript-tdf-library/go-sdk-delivery',
    'typescript':'goalchemy/out/typescript-tdf-library/sdk-delivery-http-cleanup',
    'java':'goalchemy/out/java-tdf-library/sdk-launch-repaired',
    'csharp':'goalchemy/out/csharp-tdf-library/sdk-final-sealed',
    'python':'sdk/.local/python-tdf-library-acceptance-repair/package-a',
    'rust':'goalchemy/out/rust-tdf-library/acceptance-repair-pure-sdk',
    'c':'goalchemy/out/c-tdf-library/sdk-review-verified',
}


def module(name):
    spec = importlib.util.spec_from_file_location(name,SDK/'scripts'/('delivery-'+name+'.py'))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


packages,coverage = module('packages'),module('coverage')


def identity(path):
    return {'path':str(path),'sha256':packages.sha(path)}


def keys(result):
    out = set()
    for row in result['pairs']:
        direction = coverage.direction(row)
        if direction == 'self':
            continue
        out.add((row['case'],row.get('wrap',row.get('wrapping','rsa:2048')),row.get('session','rsa:2048'),row.get('auth','ES256'),direction))
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace',type=Path,default=SDK.parent)
    parser.add_argument('--packages-base',type=Path,required=True)
    parser.add_argument('--focused-base',type=Path,required=True)
    parser.add_argument('--receipt',type=Path,required=True)
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    targets = {}
    matrices = []
    for target,tree in TREES.items():
        accepted = workspace/tree
        final = args.packages_base/'packages'/target/'relative'
        accepted_sources,final_sources = packages.production_sources(accepted,target),packages.production_sources(final,target)
        raw,material = {},{}
        for name in sorted(set(accepted_sources)|set(final_sources)):
            if accepted_sources.get(name) == final_sources.get(name):
                continue
            change = {'accepted':accepted_sources.get(name),'final':final_sources.get(name)}
            raw[name] = change
            if target == 'go' and name == 'main.go' and name in accepted_sources and name in final_sources:
                strip = lambda path: re.sub(rb'/\*line [^*]*\*/',b'',path.read_bytes())
                if strip(accepted/name) == strip(final/name):
                    change['only_diagnostic_source_positions'] = True
                    continue
            material[name] = change
        assert final_sources and not material, target+' material source drift '+str(material)
        acceptance = workspace/'sdk/.local'/('root-'+target+'-library-accepted-evidence.json')
        targets[target] = {'accepted_final_artifact_tree':str(accepted),'acceptance':identity(acceptance),
                           'package_receipt':identity(args.packages_base/'packages'/target/'receipt.json'),
                           'consumer_receipt':identity(args.packages_base/'consumers'/target/'receipt.json'),
                           'production_source_members':final_sources,'raw_deltas':raw,'material_deltas':material}
    for target in (*packages.TARGETS,'typescript-browser'):
        browser = target == 'typescript-browser'
        for profile in ('basic','ec','dpop'):
            historical = workspace/'sdk/.local'/('typescript-tdf-library' if browser else target+'-tdf-library')/(('browser-'+profile) if browser else profile)/'results.json'
            if target == 'java' and profile == 'basic':
                historical = historical.parent.parent/'basic-result-fix/results.json'
            result = json.loads(historical.read_text())
            old_keys = keys(result)
            focused = args.focused_base/target/profile/'results.json'
            new_keys = keys(json.loads(focused.read_text())) if focused.exists() else set()
            combinations = [('rsa:2048','rsa:2048','ES256')] if profile == 'basic' else list(itertools.product(('rsa:2048','ec:secp256r1'),('rsa:2048','ec:secp256r1'),('ES256','RS256')))
            directions = ['target->stock-go','stock-go->target','stock-web->target']+(['target->stock-web'] if profile != 'dpop' else [])
            expected = {(case,w,s,a,d) for case in coverage.CASES for w,s,a in combinations for d in directions}
            assert not expected-(old_keys|new_keys), target+' '+profile+' canonical gap '+str(expected-(old_keys|new_keys))
            limitations = result.get('stock_web_limitations',[])
            if profile == 'dpop' and not browser:
                assert {(row['wrapping'],row['auth']) for row in limitations} == set(itertools.product(('rsa:2048','ec:secp256r1'),('ES256','RS256')))
                assert all(row['category']=='authentication401' and not row['success'] and row['exit'] != 0 for row in limitations)
            matrices.append({'target':target,'profile':profile,'historical_result':identity(historical),
                             'historical_required_comparisons':len(expected&old_keys),
                             'focused_result':identity(focused) if focused.exists() else None,
                             'newly_closed_required_comparisons':[list(k) for k in sorted(expected-old_keys)],
                             'required_comparisons':[list(k) for k in sorted(expected)],'missing':[],
                             'known_reference_failures':limitations,'scope':'historical accepted full matrices plus separately attributed focused final-package checks'})
    args.receipt.parent.mkdir(parents=True,exist_ok=True)
    args.receipt.write_text(json.dumps({'status':0,'scope':'read-only accepted source and canonical case correspondence; not a fresh full matrix or remote CI execution',
                                      'targets':targets,'matrices':matrices,'full_matrix_replayed':False,'missing_required_cases':[]},indent=2)+'\n')
    print('PASS accepted source correspondence and combined canonical coverage',len(matrices),'environments')


if __name__ == '__main__':
    main()
