#!/usr/bin/env python3
"""Independent ZIP mutations only; no TDF encryption/decryption implementation."""
import sys,io,json,zipfile
from pathlib import Path
archive=Path(sys.argv[1]).read_bytes();destination=Path(sys.argv[2])
with zipfile.ZipFile(io.BytesIO(archive)) as z:original={n:z.read(n) for n in z.namelist()}
manifest_name='manifest.json' if 'manifest.json' in original else '0.manifest.json'
for name in ['tampered','root-tamper','unsupported-root','unsupported-assertion','untrusted']:
    entries=dict(original);manifest=json.loads(entries[manifest_name]);integrity=manifest['encryptionInformation']['integrityInformation']
    if name=='tampered':
        payload=bytearray(entries['0.payload']);payload[12]^=1;entries['0.payload']=payload
    elif name=='root-tamper':
        sig=integrity['rootSignature']['sig'];integrity['rootSignature']['sig']=('A' if sig[0]!='A' else 'B')+sig[1:]
    elif name=='unsupported-root':integrity['rootSignature']['alg']='GMAC'
    elif name=='unsupported-assertion':manifest['assertions']=[{'id':'mandatory'}]
    else:manifest['encryptionInformation']['keyAccess'][0]['url']='http://localhost:8081/kas'
    if name!='tampered':entries[manifest_name]=json.dumps(manifest).encode()
    with zipfile.ZipFile(destination/(name+'.tdf'),'w',compression=zipfile.ZIP_STORED) as z:
        for n,b in entries.items():z.writestr(n,b)
