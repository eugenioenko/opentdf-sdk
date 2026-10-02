#!/usr/bin/env python3
"""Assert health, issuer/audience, authenticated RSA discovery; save local evidence."""
import base64
import json
from pathlib import Path
from urllib.error import HTTPError
from urllib.parse import urlencode
from urllib.request import Request, urlopen

LOCAL = Path(__file__).resolve().parents[1] / '.local'
(LOCAL / 'platform-check.json').unlink(missing_ok=True)

def request(url, data=None, headers=None):
    with urlopen(Request(url, data=data, headers=headers or {}), timeout=15) as response:
        return json.load(response)

health = request('http://localhost:8080/healthz')
try:
    request('http://localhost:8080/policy.attributes.AttributesService/ListAttributes', b'{}',
            {'Content-Type': 'application/json', 'Connect-Protocol-Version': '1'})
except HTTPError as error:
    assert error.code == 401, error.code
else:
    raise AssertionError('Unauthenticated protected request was accepted')
discovery = request('http://localhost:8888/auth/realms/opentdf/.well-known/openid-configuration')
issuer = 'http://localhost:8888/auth/realms/opentdf'
assert discovery['issuer'] == issuer, discovery['issuer']
token = request(discovery['token_endpoint'], urlencode({
    'grant_type': 'client_credentials', 'client_id': 'opentdf-sdk', 'client_secret': 'secret'
}).encode(), {'Content-Type': 'application/x-www-form-urlencoded'})
assert token['token_type'].lower() == 'bearer', token['token_type']
part = token['access_token'].split('.')[1]
claims = json.loads(base64.urlsafe_b64decode(part + '=' * (-len(part) % 4)))
assert claims['iss'] == issuer
assert 'http://localhost:8080' in claims['aud']
key = request('http://localhost:8080/kas.AccessService/PublicKey',
              b'{"algorithm":"rsa:2048"}', {
                  'Authorization': 'Bearer ' + token['access_token'],
                  'Content-Type': 'application/json', 'Connect-Protocol-Version': '1'
              })
assert key['kid'] == 'r1', key
assert 'BEGIN' in key['publicKey'], key
report = {'health': health, 'issuer': claims['iss'], 'audience': claims['aud'],
          'token_type': token['token_type'], 'kas_public_key': key,
          'authentication_enabled': True, 'unauthenticated_policy_status': 401, 'dpop_enforced': False,
          'ec_tdf_enabled': False, 'kas_url': 'http://localhost:8080/kas'}
(LOCAL / 'platform-check.json').write_text(json.dumps(report, indent=2) + '\n')
print('PASS health, client-credentials Bearer token issuer/audience, real KAS RSA key r1')
