#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Actual local-server native HTTP contract checks; no KAS substitution."""
import gzip
import hashlib
import http.server
import json
import os
import pathlib
import subprocess
import sys
import threading
import zlib

consumer, base = pathlib.Path(sys.argv[1]).resolve(), pathlib.Path(sys.argv[2]).resolve()
base.mkdir(parents=True, exist_ok=True)
body = b'bounded decompressed HTTP body\x00\xff' * 128
rows = []
cases = ('nul-header', 'nul-value', 'gzip', 'gzip-bound', 'sorted-headers',
         'explicit-gzip', 'explicit-custom', 'empty-encoding', 'range',
         'unsolicited-deflate', 'chunked-gzip')
for mode in cases:
    run = base / mode
    run.mkdir()
    contacts = []
    compressed = mode not in ('nul-header', 'nul-value', 'sorted-headers')
    encoded = zlib.compress(body) if mode == 'unsolicited-deflate' else gzip.compress(body) if compressed else b'plain'

    class Handler(http.server.BaseHTTPRequestHandler):
        protocol_version = 'HTTP/1.1'

        def log_message(self, *args):
            pass

        def do_GET(self):
            contacts.append({'path': self.path, 'headers': list(self.headers.items())})
            self.send_response(200)
            self.send_header('X-Zebra', 'first')
            self.send_header('X-Alpha', 'alpha')
            self.send_header('x-zebra', 'second')
            if mode == 'chunked-gzip':
                self.send_header('Transfer-Encoding', 'chunked')
            else:
                self.send_header('Content-Length', str(len(encoded)))
            if compressed:
                self.send_header('Content-Encoding', 'deflate' if mode == 'unsolicited-deflate' else 'gzip')
            self.end_headers()
            if mode == 'chunked-gzip':
                for chunk in (encoded[:17], encoded[17:]):
                    self.wfile.write(('%x\r\n' % len(chunk)).encode() + chunk + b'\r\n')
                self.wfile.write(b'0\r\n\r\n')
            else:
                self.wfile.write(encoded)

    server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    thread = threading.Thread(target=server.serve_forever)
    thread.start()
    command = [str(consumer), 'http://127.0.0.1:%d/probe' % server.server_port,
               mode, '128' if mode == 'gzip-bound' else '8192', str(run)]
    try:
        result = subprocess.run(command, capture_output=True, timeout=15, env=os.environ)
        (run / 'consumer.log').write_bytes(result.stdout + result.stderr)
        (run / 'consumer.status').write_text(str(result.returncode) + '\n')
        assert result.returncode == 0
        kind, status = int((run / 'kind').read_text()), int((run / 'status').read_text())
        actual = (run / 'body').read_bytes()
        flat = (run / 'headers').read_text().splitlines()
        headers = list(zip(flat[::2], flat[1::2]))
        names = [n for n, _ in headers]
        if mode in ('nul-header', 'nul-value'):
            passed = kind == 1 and not contacts and status == 0 and not actual
        elif mode == 'gzip-bound':
            passed = kind == 1 and status == 0 and not actual
        else:
            automatic = mode in ('gzip', 'empty-encoding', 'chunked-gzip')
            passed = kind == 0 and status == 200 and actual == (body if automatic else encoded)
            passed &= names == sorted(names) and [v for n, v in headers if n == 'X-Zebra'] == ['first', 'second']
            passed &= ('Content-Encoding' not in names and 'Content-Length' not in names) if automatic else (not compressed or 'Content-Encoding' in names)
            request_headers = {n.lower(): v for n, v in contacts[0]['headers']}
            expected_encoding = 'gzip' if mode != 'explicit-custom' else 'br, gzip'
            if mode == 'range':
                passed &= 'accept-encoding' not in request_headers and request_headers.get('range') == 'bytes=0-100'
            else:
                passed &= request_headers.get('accept-encoding') == expected_encoding
        (run / 'contract.status').write_text(('0' if passed else '1') + '\n')
        rows.append({'case': mode, 'command': command, 'timeout_seconds': 15,
                     'consumer_status': result.returncode, 'contract_status': 0 if passed else 1,
                     'kind': kind, 'status': status, 'contacts': contacts, 'headers': headers,
                     'body_bytes': len(actual), 'body_sha256': hashlib.sha256(actual).hexdigest(),
                     'expected_body_sha256': hashlib.sha256(body if mode in ('gzip', 'empty-encoding', 'chunked-gzip') else encoded).hexdigest()})
        print(mode, 'contract', 0 if passed else 1, 'kind', kind, 'contacts', len(contacts), flush=True)
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
(base / 'results.json').write_text(json.dumps({'scope': __doc__, 'cases': rows}, indent=2) + '\n')
assert all(row['contract_status'] == 0 for row in rows)
