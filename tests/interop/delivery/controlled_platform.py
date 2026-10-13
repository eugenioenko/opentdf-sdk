#!/usr/bin/env python3
"""Offline controlled checks for the compiled native consumers (Java, C#, Rust,
C, Swift): the twin of the Go consumer's negative.go. The harness runs a
platform that only discovers and rejects (no KAS), one fixture per case, and
drives the consumer in `controlled <case>` mode; the consumer performs the
SDK call through the installed package and writes `<case>.error.json`. The
`offline` case runs the allowlist, input, option and handle checks on a TDF
the consumer writes against an explicit KAS key."""
import http.server
import json
import os
import ssl
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

SDK = Path(__file__).resolve().parents[3]
CASES = ('unreachable', 'wellknown-500', 'redirect', 'content-type', 'malformed-response', 'untrusted-tls', 'caller-deadline', 'source-deadline', 'token-401', 'offline')
# Each target reports an interrupted caller deadline in its cancellation idiom:
# a Java Duration expires as deadline_exceeded; a C# CancellationToken, a
# dropped Rust future, a C atomic flag and a cancelled Swift Task as canceled.
CALLER_DEADLINE_KIND = {'java': 'deadline_exceeded', 'csharp': 'canceled', 'rust': 'canceled', 'c': 'canceled', 'swift': 'canceled'}
# The C runtime's libcurl transport reports its own expired request timeout
# as "http: transport failure" rather than the deadline sentinel; whichever of
# that timeout and the call context's deadline fires first decides the kind
# (a Goalchemy runtime gap, recorded in docs/progress.md).
SOURCE_DEADLINE_KIND = {'java': ('deadline_exceeded',), 'csharp': ('deadline_exceeded',), 'rust': ('deadline_exceeded',), 'c': ('deadline_exceeded', 'transport'), 'swift': ('deadline_exceeded',)}
WANT = {'unreachable': 'transport', 'wellknown-500': 'transport', 'redirect': 'transport', 'content-type': 'transport', 'malformed-response': 'transport', 'untrusted-tls': 'transport', 'source-deadline': 'deadline_exceeded', 'token-401': 'auth'}
OFFLINE = {'allowlist': 'allowlist_rejected', 'invalid-tdf': 'not_decryptable', 'unsupported-option': 'unsupported_feature', 'closed-handle': 'closed'}


def jsonbytes(v):
    return json.dumps(v, separators=(',', ':')).encode()


class Fixture(http.server.BaseHTTPRequestHandler):
    """A platform that only discovers and rejects: no KAS."""
    mode = ''
    state = None

    def log_message(self, *args):
        pass

    def reply(self, status, body, content_type='application/json', extra=()):
        self.send_response(status)
        self.send_header('Content-Type', content_type)
        self.send_header('Content-Length', str(len(body)))
        for k, v in extra:
            self.send_header(k, v)
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        self.state['requests'] += 1
        self.rfile.read(int(self.headers.get('Content-Length', '0')))
        issuer = 'http://' + self.headers['Host'] + '/issuer'
        mode = self.mode
        well_known = self.path == '/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration'
        if mode == 'wellknown-500' and well_known:
            self.reply(500, b'{"code":"internal","message":"harmless fixture diagnostic"}')
        elif mode == 'redirect' and well_known:
            self.reply(307, b'', extra=[('Location', '/leak')])
        elif mode == 'content-type' and well_known:
            self.reply(200, b'{}', 'text/html')
        elif mode == 'malformed-response' and well_known:
            self.reply(200, b'{bad')
        elif mode in ('caller-deadline', 'source-deadline') and well_known:
            self.state['entered'].set()
            try:
                self.connection.settimeout(15)
                self.rfile.read(1)
            except OSError:
                pass
            self.state['released'].set()
            self.close_connection = True
        elif well_known:
            self.reply(200, jsonbytes({'configuration': {'platform_issuer': issuer, 'idp': {'issuer': issuer, 'token_endpoint': issuer + '/protocol/openid-connect/token'}}}))
        elif self.path == '/issuer/protocol/openid-connect/token':
            self.reply(401, b'{"error":"invalid_client","error_description":"harmless fixture diagnostic"}')
        elif self.path == '/grpc.health.v1.Health/Check':
            self.reply(200, b'{"status":"SERVING"}')
        elif self.path == '/policy.kasregistry.KeyAccessServerRegistryService/ListKeyAccessServers':
            self.reply(200, b'{"keyAccessServers":[]}')
        else:
            self.reply(404, jsonbytes({'code': 'unimplemented', 'message': self.path}))

    def do_GET(self):
        self.state['requests'] += 1
        issuer = 'http://' + self.headers['Host'] + '/issuer'
        if self.path == '/issuer/.well-known/openid-configuration':
            self.reply(200, jsonbytes({'token_endpoint': issuer + '/protocol/openid-connect/token'}))
        else:
            self.reply(404, b'')


def serve(mode, tls=None):
    state = {'requests': 0, 'entered': threading.Event(), 'released': threading.Event()}
    handler = type('Handler', (Fixture,), {'mode': mode, 'state': state})
    server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), handler)
    if tls:
        server.socket = tls.wrap_socket(server.socket, server_side=True)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server, state, ('https' if tls else 'http') + '://127.0.0.1:' + str(server.server_port)


def run(target, consumer_command, base, cases=CASES):
    """Runs every case through consumer_command (the consumer's argv prefix)
    and writes base/results.json; raises on the first failure."""
    base = Path(base)
    base.mkdir(parents=True, exist_ok=True)
    rows = []
    with tempfile.TemporaryDirectory() as tmp:
        key, cert = Path(tmp) / 'key.pem', Path(tmp) / 'cert.pem'
        subprocess.run(['openssl', 'req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-keyout', str(key), '-out', str(cert), '-days', '1', '-subj', '/CN=localhost', '-addext', 'subjectAltName=IP:127.0.0.1'], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        tls = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        tls.load_cert_chain(str(cert), str(key))
        kas_key = Path(tmp) / 'kas.pem'
        subprocess.run(['openssl', 'genpkey', '-algorithm', 'RSA', '-pkeyopt', 'rsa_keygen_bits:2048', '-out', str(kas_key)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        kas_public = subprocess.run(['openssl', 'pkey', '-in', str(kas_key), '-pubout'], check=True, stdout=subprocess.PIPE).stdout.decode()
        closed = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Fixture)
        closed_url = 'http://127.0.0.1:' + str(closed.server_port)
        closed.server_close()
        for case in cases:
            run_dir = base / case
            run_dir.mkdir(exist_ok=False)
            if case == 'unreachable':
                server, state, url = None, {'requests': 0, 'entered': threading.Event(), 'released': threading.Event()}, closed_url
            else:
                mode = case if case != 'offline' else ''
                server, state, url = serve(mode, tls if case == 'untrusted-tls' else None)
            (run_dir / 'config.json').write_text(json.dumps({'PlatformURL': url, 'Case': case, 'KASPublicKeyPEM': kas_public}))
            start = time.monotonic()
            try:
                with (run_dir / 'consumer.log').open('wb') as log:
                    result = subprocess.run([*consumer_command, str(SDK), str(run_dir), 'controlled', case], stdout=log, stderr=subprocess.STDOUT, timeout=60)
                elapsed = time.monotonic() - start
                (run_dir / 'consumer.status').write_text(str(result.returncode) + '\n')
                if result.returncode:
                    raise RuntimeError('controlled case failed ' + case + '; see ' + str(run_dir / 'consumer.log'))
                if case == 'offline':
                    report = json.loads((run_dir / 'offline.json').read_text())
                    for label, want in OFFLINE.items():
                        got = report[label]['kind']
                        assert got == want, 'controlled wrong kind: %s: %s (%s)' % (label, got, report[label])
                        rows.append({'case': label, **report[label]})
                    assert 'tdf: decrypt failed' in report['allowlist']['message'], 'allowlist rejection is a decrypt failure'
                    assert 'tdf: input is not decryptable' in report['invalid-tdf']['message'], 'invalid input sentinel'
                    if 'invalid-argument' in report:
                        assert report['invalid-argument']['kind'] == 'invalid_argument', report['invalid-argument']
                        rows.append({'case': 'invalid-argument', **report['invalid-argument']})
                    assert report['handles']['readers'] == 100 and report['handles']['closed'], report['handles']
                    rows.append({'case': 'handles', **report['handles']})
                    continue
                error = json.loads((run_dir / (case + '.error.json')).read_text())
                want = (CALLER_DEADLINE_KIND[target],) if case == 'caller-deadline' else SOURCE_DEADLINE_KIND[target] if case == 'source-deadline' else (WANT[case],)
                assert error['kind'] in want, 'controlled wrong kind: %s: %s (%s)' % (case, error['kind'], error)
                if case == 'unreachable':
                    assert 'platform unreachable or not responding' in error['message'], 'unreachable platform sentinel'
                if case not in ('unreachable', 'caller-deadline', 'token-401', 'untrusted-tls'):
                    assert 'failed to retrieve platform configuration' in error['message'], 'discovery failures carry ErrPlatformConfigFailed: ' + case
                if case == 'token-401':
                    assert error['httpStatus'] == 401 and error['connectCode'] == 'invalid_client' and 'tdf: decrypt failed' in error['message'], 'token failure fields'
                if case in ('caller-deadline', 'source-deadline'):
                    assert elapsed < 10, 'deadline delayed'
                    # A deadline that expires before the request reaches the
                    # fixture leaves nothing to release; otherwise the socket
                    # must be observed closed before the consumer exits.
                    if state['entered'].is_set():
                        assert state['released'].wait(3), 'HTTP request cleanup not observed: ' + case
                    else:
                        assert state['requests'] == 0, 'deadline after contact without release: ' + case
                if case == 'untrusted-tls':
                    assert state['requests'] == 0, 'untrusted TLS reached handler'
                rows.append({'case': case, **error, 'requests': state['requests'], 'elapsed_ms': int(elapsed * 1000)})
                print('PASS generated', target, 'controlled', case, flush=True)
            finally:
                if server is not None:
                    server.shutdown()
                    server.server_close()
    (base / 'results.json').write_text(json.dumps({'scope': 'controlled discovery, transport, allowlist, option and handle checks through actual generated exports; no mock positive KAS', 'negatives': rows}, indent=2) + '\n')
    print('PASS generated', target, 'controlled:', len(rows), 'cases', flush=True)


def main(target, consumer_command):
    base = Path(os.environ.get('TDF_' + target.upper() + '_CONTROLLED_OUT', str(SDK / ('.local/' + target + '-tdf-library/controlled'))))
    cases = os.environ.get('TDF_' + target.upper() + '_CONTROLLED_CASES')
    run(target, consumer_command, base, cases.split(',') if cases else CASES)


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2:])
