import { test } from 'node:test';
import assert from 'node:assert/strict';
import { measureWeb } from './web-node.mjs';
const config = { KASURL: 'http://localhost:8080/kas', KID: 'key', KASPublicKeyPEM: 'public' };
function fakeClient(trace, mismatch = false) {
  return {
    async encrypt(options) {
      assert.equal(options.windowSize, 2 << 20);
      assert.equal(options.segmentIntegrityAlgorithm, 'GMAC');
      assert.equal(options.wrappingKeyAlgorithm, 'rsa:2048');
      const input = new Uint8Array(await new Response(options.source).arrayBuffer());
      return { stream: new ReadableStream({ pull(controller) { trace.push('encrypted-consumed'); controller.enqueue(input.slice()); controller.close(); } }) };
    },
    async decrypt(options) {
      assert.equal(options.wrappingKeyAlgorithm, 'rsa:2048');
      const archive = options.source.location;
      trace.push('decrypt-owned-archive');
      return { stream: new ReadableStream({ pull(controller) { trace.push('plaintext-consumed'); controller.enqueue(mismatch ? new Uint8Array([0]) : archive.slice()); controller.close(); } }) };
    },
  };
}
test('contiguous timer includes complete encrypt and decrypt streams; only final actual warmup retained', async () => {
  const trace = [], saved = [];
  const result = await measureWeb(fakeClient(trace), config, new Uint8Array([1]), new Uint8Array([2, 3]), { samples: 2, warmups: 2, bulkWarmups: 1 }, async (i, archive) => { trace.push('save'); saved.push([i, [...archive]]); }, () => { trace.push('clock'); return trace.length; });
  assert.equal(result.correct, true);
  assert.deepEqual(saved, [[-1, [1]], [0, [1]], [1, [1]]]);
  assert.equal(result.bulk_warmup_ms.length, 1);
  assert.equal(result.warmup_ms.length, 2);
  assert.equal(result.samples_ms.length, 2);
  assert.equal(result.kas_calls_expected, 5);
  for (let i = 0; i < trace.length; i++) if (trace[i] === 'encrypted-consumed') {
    assert.equal(trace[i-1], 'clock');
    assert.deepEqual(trace.slice(i+1,i+4), ['decrypt-owned-archive','plaintext-consumed','clock']);
  }
});
test('mismatched plaintext fails before result publication/archive retention', async () => {
  let saves = 0;
  await assert.rejects(measureWeb(fakeClient([], true), config, new Uint8Array([1]), new Uint8Array([2]), { samples: 1, warmups: 1, bulkWarmups: 0 }, async () => saves++), /plaintext mismatch/);
  assert.equal(saves, 0);
});
test('invalid native policy is rejected before invoking SDK', async () => {
  await assert.rejects(measureWeb({}, config, new Uint8Array(), new Uint8Array(), { samples: 0, warmups: 1, bulkWarmups: 0 }, async () => {}));
});
