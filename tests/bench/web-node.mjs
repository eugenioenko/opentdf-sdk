/** Public pinned stock Web SDK in Node; CLI/OAuth/discovery are outside pair timers. */
import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

export async function measureWeb(client, config, input, bulkInput, policy, save, clock = () => performance.now()) {
  const { samples, warmups, bulkWarmups } = policy;
  assert(Number.isInteger(samples) && samples > 0 && Number.isInteger(warmups) && warmups > 0 && Number.isInteger(bulkWarmups) && bulkWarmups >= 0);
  const history = { samples_ms: [], warmup_ms: [], bulk_warmup_ms: [] };
  const attribute = 'https://example.com/attr/attr1/value/value1';
  const scope = { attributes: [attribute], attributeValues: [{ fqn: attribute, kasKeys: [{ kasUri: config.KASURL, publicKey: { algorithm: 1, kid: config.KID, pem: config.KASPublicKeyPEM } }] }] };
  for (let i = -bulkWarmups - warmups; i < samples; i++) {
    const plaintext = i < -warmups ? bulkInput : input;
    const start = clock();
    const encrypted = await client.encrypt({ source: new Blob([plaintext]).stream(), scope, defaultKASEndpoint: config.KASURL, autoconfigure: false, windowSize: 2 << 20, wrappingKeyAlgorithm: 'rsa:2048', splitPlan: [{ kas: config.KASURL, kid: config.KID }], rootIntegrityAlgorithm: 'HS256', segmentIntegrityAlgorithm: 'GMAC' });
    const archive = new Uint8Array(await new Response(encrypted.stream).arrayBuffer());
    const decrypted = await client.decrypt({ source: { type: 'buffer', location: archive }, wrappingKeyAlgorithm: 'rsa:2048' });
    const output = new Uint8Array(await new Response(decrypted.stream).arrayBuffer());
    const elapsed = clock() - start;
    assert(Buffer.from(output).equals(Buffer.from(plaintext)), 'stock Web plaintext mismatch');
    if (i === -1 || i >= 0) await save(i, archive);
    history[i >= 0 ? 'samples_ms' : i < -warmups ? 'bulk_warmup_ms' : 'warmup_ms'].push(elapsed);
  }
  return { ...history, correct: true, warmup_count: warmups, bulk_warmup_count: bulkWarmups, kas_calls_expected: samples + warmups + bulkWarmups, client_initializations: 1, signing_algorithm: 'ES256', authentication: 'Bearer', response_session_algorithm: 'rsa:2048', response_key_lifecycle: 'stock SDK fresh RSA2048 key inside each decrypt' };
}

async function main() {
  const [run, operation, size, n, warm = '20', bulk = '40'] = process.argv.slice(2);
  assert(operation === 'e2e');
  const raw = JSON.parse(await readFile(join(run, 'private.json')));
  const input = await readFile(join(run, size + '.input'));
  const bulkInput = Number(bulk) && size !== '50' ? await readFile(join(run, '50.input')) : input;
  const require = createRequire(join(resolve(process.env.TDF_WEB_PACKAGE), 'package.json'));
  const { TDF3Client, authTokenInterceptor } = require('@opentdf/sdk');
  const { WebCryptoService } = require('@opentdf/sdk/singlecontainer');
  const pem = generateKeyPairSync('ec', { namedCurve: 'prime256v1', publicKeyEncoding: { type: 'spki', format: 'pem' }, privateKeyEncoding: { type: 'pkcs8', format: 'pem' } });
  const keyOptions = { usage: 'sign', algorithmHint: 'ec:secp256r1' };
  const keys = { publicKey: await WebCryptoService.importPublicKey(pem.publicKey, keyOptions), privateKey: await WebCryptoService.importPrivateKey(pem.privateKey, keyOptions) };
  const client = new TDF3Client({ interceptors: [authTokenInterceptor(async () => raw.Token)], dpopEnabled: false, dpopKeys: Promise.resolve(keys), kasEndpoint: raw.Config.KASURL, platformUrl: raw.Config.PlatformURL, policyEndpoint: raw.Config.PlatformURL, allowedKases: [raw.Config.KASURL] });
  await client.dpopKeys;
  const result = await measureWeb(client, raw.Config, input, bulkInput, { samples: Number(n), warmups: Number(warm), bulkWarmups: Number(bulk) }, (i, archive) => writeFile(join(run, `web-${size}-${i}.tdf`), archive));
  console.log(JSON.stringify(result));
}
if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) await main();
