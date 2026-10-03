// One contiguous public encrypt/decrypt interval; harness setup stays outside.
export async function measure(g, raw, input, archive, op, n, save) {
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
  const samples = [];
  for (let i = -1; i < n; i++) {
    const start = performance.now();
    const encrypted = await g.encrypt(cfg, input, options, call);
    const output = (await g.decrypt(cfg, encrypted, call)).Payload;
    const elapsed = performance.now() - start;
    if (
      output.length !== input.length ||
      !output.every((v, j) => v === input[j])
    )
      throw new Error("plaintext mismatch");
    await save(i, encrypted);
    if (i >= 0) samples.push(elapsed);
  }
  return { samples_ms: samples, correct: true, kas_calls_expected: n + 1 };
}
