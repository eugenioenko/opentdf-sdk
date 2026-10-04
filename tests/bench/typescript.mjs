// One contiguous public encrypt/decrypt interval; harness setup stays outside.
export async function measure(g, raw, input, archive, op, n, save, policy = {}) {
  if (op !== "e2e") throw new Error("TypeScript consumer supports e2e only");
  const cfg = { ...raw.Config };
  const call = {
    tokenProvider: async () => ({
      value: raw.Token,
      scheme: "Bearer",
      expiresAt: BigInt(raw.Expires),
    }),
  };
  const options = {
    Attributes: ["https://example.com/attr/attr1/value/value1"],
    SegmentSize: 2097152n,
    HasSegmentSize: true,
    SegmentHashAlgorithm: "GMAC",
  };
  const { warmups = 1, bulkWarmups = 0, bulkInput = input } = policy;
  if (n < 1 || warmups < 1 || bulkWarmups < 0) throw new Error("invalid samples or warmups");
  const samples = [], warmupHistory = [], bulkHistory = [];
  for (let i = -bulkWarmups - warmups; i < n; i++) {
    const pairInput = i < -warmups ? bulkInput : input;
    const start = performance.now();
    const encrypted = await g.encrypt(cfg, pairInput, options, call);
    const output = (await g.decrypt(cfg, encrypted, call)).Payload;
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
  return { samples_ms: samples, warmup_ms: warmupHistory, bulk_warmup_ms: bulkHistory, warmup_count: warmups, bulk_warmup_count: bulkWarmups, correct: true, kas_calls_expected: n + warmups + bulkWarmups };
}
