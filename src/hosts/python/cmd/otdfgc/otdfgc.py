#!/usr/bin/env python3
"""otdfgc is the native Python CLI over the generated package. It implements
the xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc does:
one SDK per invocation, created with --host and the client credentials; every
flag this build cannot honour exits 2 as unsupported."""
import json
import os
import sys
import zipfile

import opentdf_tdf3 as tdf

EXIT_SUCCESS, EXIT_ERROR, EXIT_USAGE = 0, 1, 2
TARGET = 'python'
TIMEOUT_SECONDS = 120
USAGE = '''Usage:
  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--with-assertions <file|json>] [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--no-verify-assertions] [--with-assertion-verification-keys <file|json>] [--session-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc inspect <in>
  otdfgc supports <feature>
  otdfgc --version [--json]
'''

COMMON_FLAGS = (('out', False), ('host', False), ('with-client-creds', False), ('with-client-creds-file', False), ('dpop', False), ('dpop-key', False), ('tls-no-verify', True), ('log-level', False))
ENCRYPT_FLAGS = (('attr', False), ('mime-type', False), ('with-assertions', False), ('wrapping-key-algorithm', False), ('target-mode', False), ('policy-mode', False), ('ecdsa-binding', True)) + COMMON_FLAGS
DECRYPT_FLAGS = (('with-assertion-verification-keys', False), ('no-verify-assertions', True), ('session-key-algorithm', False), ('kas-allowlist', False)) + COMMON_FLAGS
DPOP_ALGORITHMS = ('ES256', 'ES384', 'ES512', 'RS256', 'RS384', 'RS512')
ENCRYPT_UNSUPPORTED = (('ecdsa-binding', None), ('target-mode', ('4.3.0',)), ('policy-mode', ('encrypted',)), ('wrapping-key-algorithm', ('rsa:2048', 'ec:secp256r1', 'rsa:4096', 'ec:secp384r1', 'ec:secp521r1')), ('dpop', DPOP_ALGORITHMS), ('with-client-creds-file', None))
DECRYPT_UNSUPPORTED = (('session-key-algorithm', ('rsa:2048', 'ec:secp256r1', 'rsa:4096', 'ec:secp384r1', 'ec:secp521r1')), ('dpop', DPOP_ALGORITHMS), ('with-client-creds-file', None))


class Parsed:
    def __init__(self):
        self.values, self.set, self.positional = {}, set(), []
    def value(self, name): return self.values.get(name, '')
    def flag(self, name): return name in self.set


def parse_flags(args, spec):
    specs = dict(spec)
    p = Parsed()
    i = 0
    while i < len(args):
        a = args[i]
        if not a.startswith('-') or a == '-':
            p.positional.append(a); i += 1; continue
        name = a.lstrip('-'); value = ''; inline = False
        if '=' in name:
            name, value = name.split('=', 1); inline = True
        if name == 'o': name = 'out'
        if name not in specs: raise ValueError('unknown flag: ' + a)
        if specs[name]:
            if inline and value not in ('true', 'false'): raise ValueError('flag --%s takes no value' % name)
            if not inline or value == 'true': p.set.add(name)
            i += 1; continue
        if not inline:
            if i + 1 >= len(args): raise ValueError('flag --%s needs a value' % name)
            i += 1; value = args[i]
        p.values[name] = value; p.set.add(name); i += 1
    return p


def unsupported(p, rules):
    for name, allowed in rules:
        if name not in p.set: continue
        if allowed is None: return '--' + name
        v = p.values.get(name, '')
        if v not in allowed: return '--' + name + ' ' + v
    return ''


def split_list(s): return [part.strip() for part in s.split(',') if part.strip()]


def describe(e):
    if not isinstance(e, tdf.OpenTDFError): return str(e)
    msg = e.message
    if e.required_obligations: msg += '\nrequired obligations: [' + ' '.join(e.required_obligations) + ']'
    return msg


def describe_new(e, host):
    if not isinstance(e, tdf.OpenTDFError): return str(e)
    if e.kind == 'auth': return 'Failed to authenticate with flag-provided client credentials. (' + e.message + ')'
    if 'platform unreachable or not responding' in e.message or e.kind in ('transport', 'deadline_exceeded'):
        return "Failed to connect to the platform. Is the platform accepting connections at '%s'? (%s)" % (host, e.message)
    return e.message


def options(p):
    host = p.value('host').rstrip('/')
    if not host: return None, None, EXIT_USAGE, '--host is required\n' + USAGE
    creds = p.value('with-client-creds')
    if not creds: return None, None, EXIT_USAGE, '--with-client-creds is required\n' + USAGE
    try: c = json.loads(creds)
    except ValueError: c = None
    if type(c) is not dict or not isinstance(c.get('clientId'), str) or not c['clientId'] or not isinstance(c.get('clientSecret'), str) or not c['clientSecret']:
        return None, None, EXIT_USAGE, '--with-client-creds must be JSON with clientId and clientSecret'
    opts = {'client_id': c['clientId'], 'client_secret': c['clientSecret']}
    if host.lower().startswith('http://'): opts['insecure_plaintext'] = True
    if p.flag('tls-no-verify'): opts['insecure_skip_verify'] = True
    if p.value('dpop'): opts['dpop_algorithm'] = p.value('dpop')
    if p.value('dpop-key'):
        if not p.value('dpop'): return None, None, EXIT_USAGE, '--dpop-key needs --dpop'
        try:
            with open(p.value('dpop-key'), encoding='utf-8') as f: opts['dpop_key_pem'] = f.read()
        except OSError as e: return None, None, EXIT_ERROR, 'Failed to read DPoP key %s: %s' % (p.value('dpop-key'), e)
    return host, opts, EXIT_SUCCESS, None


def new_sdk(p, stderr):
    host, opts, code, error = options(p)
    if error is not None:
        print(error, file=stderr); return None, host, code
    try: return tdf.SDK(host, timeout=TIMEOUT_SECONDS, **opts), host, EXIT_SUCCESS
    except tdf.OpenTDFError as e:
        print(describe_new(e, host), file=stderr); return None, host, EXIT_ERROR


def write_out(path, data, mode):
    tmp = path + '.tmp'
    with open(tmp, 'wb') as f: f.write(data)
    os.chmod(tmp, mode)
    try: os.replace(tmp, path)
    except OSError:
        try: os.unlink(tmp)
        except OSError: pass
        raise


def version_info():
    return {'sdk_version': tdf.version(), 'schema_version': tdf.tdf_spec_version(), 'supported_features': list(tdf.supported_features()), 'target': TARGET}


def version(args, stdout, stderr):
    as_json = False
    for a in args:
        if a == '--json': as_json = True
        else:
            print('unknown flag: ' + a, file=stderr); return EXIT_USAGE
    try: info = version_info()
    except tdf.OpenTDFError as e:
        print('Failed to read version: ' + describe(e), file=stderr); return EXIT_ERROR
    if as_json:
        print(json.dumps(info, separators=(',', ':')), file=stdout); return EXIT_SUCCESS
    print('otdfgc (%s) sdk %s tdf-spec %s features %s' % (TARGET, info['sdk_version'], info['schema_version'], ','.join(info['supported_features'])), file=stdout)
    return EXIT_SUCCESS


def supports(args, stdout, stderr):
    if len(args) != 1 or args[0].startswith('-'):
        print('usage: otdfgc supports <feature>', file=stderr); return EXIT_USAGE
    try: features = tdf.supported_features()
    except tdf.OpenTDFError as e:
        print('Failed to read supported features: ' + describe(e), file=stderr); return EXIT_ERROR
    if args[0] in features: return EXIT_SUCCESS
    print(args[0] + ' unsupported', file=stdout)
    return EXIT_ERROR


def read_manifest(path):
    with zipfile.ZipFile(path) as archive:
        if '0.manifest.json' not in archive.namelist(): raise ValueError('manifest entry not found')
        return archive.read('0.manifest.json')


def inspect(args, stdout, stderr):
    if len(args) != 1 or args[0].startswith('-'):
        print('usage: otdfgc inspect <in>', file=stderr); return EXIT_USAGE
    try: manifest = read_manifest(args[0])
    except (OSError, ValueError, zipfile.BadZipFile) as e:
        print('not a valid TDF: ' + str(e), file=stderr); return EXIT_ERROR
    try: print(json.dumps(json.loads(manifest.decode('utf-8')), indent=2, ensure_ascii=False), file=stdout)
    except ValueError as e:
        print('failed to inspect TDF: ' + str(e), file=stderr); return EXIT_ERROR
    return EXIT_SUCCESS


def file_or_json(arg):
    """The argument's text: the file it names, or the argument itself when it
    is JSON, as otdfctl accepts both."""
    try: json.loads(arg); return arg
    except ValueError: pass
    try:
        with open(arg, encoding='utf-8') as f: return f.read()
    except OSError as e: raise ValueError('%s is neither JSON nor a readable file: %s' % (arg, e))


def assertion_key(k, label):
    if not isinstance(k, dict) or k.get('alg') not in ('HS256', 'RS256'):
        raise ValueError('%s: unsupported signing key algorithm %s' % (label, (k or {}).get('alg') if isinstance(k, dict) else k))
    return {'alg': k['alg'], 'key': str(k.get('key', ''))}


def parse_assertions(arg):
    """--with-assertions: a JSON list of assertion configs whose statement value
    is a string or an object (kept as compact JSON) and whose optional
    signingKey is {alg, key} with the HS256 secret or RS256 private key PEM."""
    try: items = json.loads(file_or_json(arg))
    except ValueError as e: raise ValueError('assertions must be a JSON list: %s' % e)
    if not isinstance(items, list): raise ValueError('assertions must be a JSON list')
    out = []
    for a in items:
        statement = a.get('statement') or {}
        c = {'id': a.get('id', ''), 'type': a.get('type', ''), 'scope': a.get('scope', ''), 'applies_to_state': a.get('appliesToState', ''),
             'statement': {'format': statement.get('format', ''), 'schema': statement.get('schema', '')}}
        value = statement.get('value')
        if isinstance(value, str): c['statement']['value'] = value
        elif isinstance(value, dict): c['statement']['value'] = json.dumps(value, separators=(',', ':'), sort_keys=True)
        elif value is not None: raise ValueError('assertion %s: statement value is neither a string nor an object' % c['id'])
        if a.get('signingKey') is not None: c['signing_key'] = assertion_key(a['signingKey'], 'assertion ' + c['id'])
        out.append(c)
    return out


def parse_verification_keys(arg):
    """--with-assertion-verification-keys: {"keys": {id: {alg, key}},
    "defaultKey": {alg, key}} with the HS256 secret or RS256 public key PEM."""
    try: data = json.loads(file_or_json(arg))
    except ValueError as e: raise ValueError('assertion verification keys must be a JSON object: %s' % e)
    if not isinstance(data, dict): raise ValueError('assertion verification keys must be a JSON object')
    keys = {}
    if data.get('defaultKey'): keys['default_key'] = {'alg': data['defaultKey'].get('alg', ''), 'key': str(data['defaultKey'].get('key', ''))}
    if data.get('keys'): keys['keys'] = {i: {'alg': k.get('alg', ''), 'key': str(k.get('key', ''))} for i, k in data['keys'].items()}
    return keys


def encrypt(args, stderr):
    try: p = parse_flags(args, ENCRYPT_FLAGS)
    except ValueError as e:
        print(str(e) + '\n' + USAGE, end='', file=stderr); return EXIT_USAGE
    reason = unsupported(p, ENCRYPT_UNSUPPORTED)
    if reason:
        print('unsupported in this build: ' + reason, file=stderr); return EXIT_USAGE
    if len(p.positional) != 1 or not p.value('out'):
        print('encrypt needs exactly one input file and --out\n' + USAGE, end='', file=stderr); return EXIT_USAGE
    try:
        with open(p.positional[0], 'rb') as f: data = f.read()
    except OSError as e:
        print('Failed to read %s: %s' % (p.positional[0], e), file=stderr); return EXIT_ERROR
    sdk, host, code = new_sdk(p, stderr)
    if sdk is None: return code
    with sdk:
        # As otdfctl: the platform's KAS is the default key access server and
        # attributes drive autoconfigure.
        opts = {'kas_information': [{'url': host + '/kas'}], 'mime_type': p.value('mime-type')}
        if p.value('attr'): opts['attributes'] = split_list(p.value('attr'))
        if p.value('wrapping-key-algorithm'): opts['wrapping_key_algorithm'] = p.value('wrapping-key-algorithm')
        if p.value('target-mode'): opts['target_mode'] = p.value('target-mode')
        if p.value('with-assertions'):
            try: opts['assertions'] = parse_assertions(p.value('with-assertions'))
            except ValueError as e:
                print('Failed to read assertions: %s' % e, file=stderr); return EXIT_USAGE
        try: out = sdk.encrypt_bytes(data, timeout=TIMEOUT_SECONDS, **opts)
        except tdf.OpenTDFError as e:
            print('Failed to encrypt: ' + describe(e), file=stderr); return EXIT_ERROR
        try: write_out(p.value('out'), out, 0o644)
        except OSError as e:
            print('Failed to write encrypted file %s: %s' % (p.value('out'), e), file=stderr); return EXIT_ERROR
    return EXIT_SUCCESS


def decrypt(args, stderr):
    try: p = parse_flags(args, DECRYPT_FLAGS)
    except ValueError as e:
        print(str(e) + '\n' + USAGE, end='', file=stderr); return EXIT_USAGE
    reason = unsupported(p, DECRYPT_UNSUPPORTED)
    if reason:
        print('unsupported in this build: ' + reason, file=stderr); return EXIT_USAGE
    if len(p.positional) != 1 or not p.value('out'):
        print('decrypt needs exactly one input file and --out\n' + USAGE, end='', file=stderr); return EXIT_USAGE
    try:
        with open(p.positional[0], 'rb') as f: data = f.read()
    except OSError as e:
        print('Failed to read file: %s' % e, file=stderr); return EXIT_ERROR
    sdk, host, code = new_sdk(p, stderr)
    if sdk is None: return code
    with sdk:
        opts = {'disable_assertion_verification': p.flag('no-verify-assertions')}
        if p.value('with-assertion-verification-keys'):
            try: opts['assertion_verification_keys'] = parse_verification_keys(p.value('with-assertion-verification-keys'))
            except ValueError as e:
                print('Failed to read assertion verification keys: %s' % e, file=stderr); return EXIT_USAGE
        if p.value('session-key-algorithm'): opts['session_key_type'] = p.value('session-key-algorithm')
        # otdfctl: "*" alone ignores the allowlist; any other list is explicit.
        allowlist = split_list(p.value('kas-allowlist'))
        if len(allowlist) == 1 and allowlist[0] == '*': opts['ignore_allowlist'] = True
        elif allowlist: opts['kas_allowlist'] = allowlist
        try: plaintext = sdk.decrypt_bytes(data, timeout=TIMEOUT_SECONDS, **opts)
        except tdf.OpenTDFError as e:
            print('Failed to decrypt file: ' + describe(e), file=stderr); return EXIT_ERROR
        try: write_out(p.value('out'), plaintext, 0o600)
        except OSError as e:
            print('Failed to write decrypted data to file: %s' % e, file=stderr); return EXIT_ERROR
    return EXIT_SUCCESS


def run(args, stdout, stderr):
    if not args:
        print(USAGE, end='', file=stderr); return EXIT_USAGE
    command, rest = args[0], args[1:]
    if command in ('--version', 'version'): return version(rest, stdout, stderr)
    if command == 'supports': return supports(rest, stdout, stderr)
    if command == 'inspect': return inspect(rest, stdout, stderr)
    if command == 'encrypt': return encrypt(rest, stderr)
    if command == 'decrypt': return decrypt(rest, stderr)
    if command in ('help', '--help', '-h'):
        print(USAGE, end='', file=stdout); return EXIT_SUCCESS
    print('unknown command: %s\n%s' % (command, USAGE), end='', file=stderr)
    return EXIT_USAGE


if __name__ == '__main__':
    sys.exit(run(sys.argv[1:], sys.stdout, sys.stderr))
