// One contiguous public encrypt/decrypt interval over the handle API; one SDK
// per process, as the reference consumer. Harness setup stays outside.
export async function measure(g, raw, input, archive, op, n, save, policy = {}) {
  if (op !== "e2e") throw new Error("TypeScript consumer supports e2e only");
  const cfg = raw.Config;
  const sdk = await g.SDK.create(cfg.PlatformURL, {
    insecurePlaintext: true,
    tokenEndpoint: cfg.TokenURL,
    tokenProvider: async () => ({
      value: raw.Token,
      scheme: "Bearer",
      expiresAt: BigInt(raw.Expires),
    }),
  });
  const options = {
    disableAutoconfigure: true,
    kasInformation: [{ url: cfg.KASURL, publicKey: cfg.KASPublicKeyPEM, kid: cfg.KID, algorithm: "rsa:2048" }],
    wrappingKeyAlgorithm: "rsa:2048",
    segmentSize: 2097152,
    attributes: ["https://example.com/attr/attr1/value/value1"],
  };
  // loadTdf reuses the SDK's default RSA2048 session. Do not generate a replacement.
  const readerOptions = { kasAllowlist: [cfg.KASURL] };
  const { warmups = 1, bulkWarmups = 0, bulkInput = input } = policy;
  if (n < 1 || warmups < 1 || bulkWarmups < 0) throw new Error("invalid samples or warmups");
  const samples = [], warmupHistory = [], bulkHistory = [];
  try {
    for (let i = -bulkWarmups - warmups; i < n; i++) {
      const pairInput = i < -warmups ? bulkInput : input;
      const start = performance.now();
      const encrypted = await sdk.encryptBytes(pairInput, options);
      const output = await sdk.decryptBytes(encrypted, readerOptions);
      const elapsed = performance.now() - start;
      if (
        output.length !== pairInput.length ||
        !output.every((v, j) => v === pairInput[j])
      )
        throw new Error("plaintext mismatch");
      if (i === -1 || i >= 0) await save(i, encrypted);
      if (i >= 0) samples.push(elapsed);
      else if (i < -warmups) bulkHistory.push(elapsed);
      else warmupHistory.push(elapsed);
    }
  } finally {
    await sdk.close();
  }
  return { samples_ms: samples, warmup_ms: warmupHistory, bulk_warmup_ms: bulkHistory, warmup_count: warmups, bulk_warmup_count: bulkWarmups, correct: true, kas_calls_expected: n + warmups + bulkWarmups };
}
