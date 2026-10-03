"""Retain per-invocation results before profile runners reuse mutable paths."""
from pathlib import Path
import hashlib
import json
import zipfile


def digest(data):
    return hashlib.sha256(data).hexdigest()


def assert_metadata_presence(run, name):
    run = Path(run)
    with zipfile.ZipFile(run/(name+'.tdf')) as archive:
        manifest = json.loads(archive.read('manifest.json' if 'manifest.json' in archive.namelist() else '0.manifest.json'))
    presence = any(bool(key.get('encryptedMetadata')) for key in manifest['encryptionInformation']['keyAccess'])
    assert (run/(name+'.presence')).read_text().strip() == str(presence).lower(), name+' metadata presence differs from immutable producer manifest'
    return presence


def capture(label, args, result, run):
    run = Path(run)
    evidence = run / 'invocations'
    objects = evidence / 'objects'
    objects.mkdir(parents=True, exist_ok=True)
    index = evidence / 'index.jsonl'
    sequence = sum(1 for _ in index.open()) if index.exists() else 0
    config = run / 'config.json'
    identity = None
    if config.exists():
        raw = config.read_bytes()
        safe = json.loads(raw)
        for field in ('ClientSecret', 'AuthPrivateKeyPEM', 'AccessToken'):
            if field in safe:
                safe[field] = '<private-input-sha256:' + digest(str(safe[field]).encode()) + '>'
        identity = {'private_input_sha256': digest(raw), 'sanitized': safe}
    files = {}
    for path in sorted(run.iterdir()):
        if path.is_file() and (path.suffix in ('.tdf', '.input', '.out', '.metadata', '.manifest', '.presence') or path.name.endswith('.error.json')):
            raw = path.read_bytes()
            sha = digest(raw)
            destination = objects / sha
            if not destination.exists():
                destination.write_bytes(raw)
            files[path.name] = {'sha256': sha, 'bytes': len(raw)}
    command = [str(arg) for arg in args]
    for i, arg in enumerate(command[:-1]):
        if arg in ('--clientSecret', '--accessToken'):
            command[i + 1] = '<private>'
    event = {'sequence': sequence, 'label': label, 'command': command,
             'terminal_status': result.returncode, 'configuration': identity, 'artifacts': files}
    with index.open('a') as stream:
        stream.write(json.dumps(event, sort_keys=True) + '\n')
    return event
