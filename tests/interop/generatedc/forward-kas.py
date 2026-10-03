#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Compress genuine BASIC KAS responses for a fresh no-preinit native importer."""
import gzip
import hashlib
import http.client
import http.server
import json
import pathlib
import subprocess
import sys
import threading

consumer, archive, expected, base = (pathlib.Path(p).resolve() for p in sys.argv[1:])
base.mkdir(parents=True, exist_ok=True)
rows = []
class Handler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass
    def exchange(self):
        request = self.rfile.read(int(self.headers.get('Content-Length', '0')))
        headers = {n: v for n, v in self.headers.items() if n.lower() not in ('host', 'connection', 'content-length', 'accept-encoding')}
        upstream = http.client.HTTPConnection('localhost', 8080, timeout=10)
        try:
            upstream.request(self.command, self.path, request, headers)
            response = upstream.getresponse()
            raw, status = response.read(), response.status
            encoded = gzip.compress(raw) if status == 200 and 'rewrap' in self.path.lower() else raw
            rows.append({'method': self.command, 'path': self.path, 'upstream_status': status,
                         'genuine_body_bytes': len(raw), 'genuine_body_sha256': hashlib.sha256(raw).hexdigest(),
                         'gzip': encoded != raw, 'wire_bytes': len(encoded), 'wire_sha256': hashlib.sha256(encoded).hexdigest(),
                         'native_accept_encoding': self.headers.get('Accept-Encoding')})
            self.send_response(status)
            for n, v in response.getheaders():
                if n.lower() not in ('connection', 'transfer-encoding', 'content-length', 'content-encoding', 'server', 'date'):
                    self.send_header(n, v)
            if encoded != raw:
                self.send_header('Content-Encoding', 'gzip')
            self.send_header('Content-Length', str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)
        finally:
            upstream.close()
    do_GET = exchange
    do_POST = exchange
server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
thread = threading.Thread(target=server.serve_forever)
thread.start()
command = [str(consumer), str(archive), 'http://127.0.0.1:%d' % server.server_port,
           str(base/'payload'), str(base/'metadata'), str(base/'presence')]
try:
    result = subprocess.run(command, capture_output=True, timeout=45)
    (base/'consumer.log').write_bytes(result.stdout+result.stderr)
    (base/'consumer.status').write_text(str(result.returncode)+'\n')
finally:
    server.shutdown()
    server.server_close()
    thread.join()
manifest = json.loads(__import__('zipfile').ZipFile(archive).read('0.manifest.json'))
presence = bool(manifest['encryptionInformation']['keyAccess'][0].get('encryptedMetadata'))
# The stock producer manifest is the independent presence oracle.
passed = result.returncode == 0 and (base/'payload').read_bytes() == expected.read_bytes() and int((base/'presence').read_text()) == presence
passed &= len([r for r in rows if r['gzip'] and r['upstream_status'] == 200]) == 2
(base/'results.json').write_text(json.dumps({'scope': __doc__, 'command': command, 'timeout_seconds': 45,
    'archive_sha256': hashlib.sha256(archive.read_bytes()).hexdigest(),
    'expected_payload_sha256': hashlib.sha256(expected.read_bytes()).hexdigest(),
    'actual_payload_sha256': hashlib.sha256((base/'payload').read_bytes()).hexdigest(),
    'producer_manifest_has_encrypted_metadata': presence, 'actual_has_metadata': int((base/'presence').read_text()),
    'metadata_bytes': len((base/'metadata').read_bytes()), 'metadata_sha256': hashlib.sha256((base/'metadata').read_bytes()).hexdigest(),
    'forwarded_real_exchanges': rows, 'contract_status': 0 if passed else 1}, indent=2)+'\n')
assert passed
print('PASS genuine BASIC KAS compressed rewrap x2, first async no-preinit native importer, payload/presence oracle and joined forwarding server')
