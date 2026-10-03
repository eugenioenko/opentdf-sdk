// Shared Node/browser loop; fixture transport and validation stay outside timing.
export async function measure(g, raw, input, archive, op, n, save) {
  const samples = [];
  for (let i = -1; i < n; i++) {
    const start = performance.now(),
      cfg = { ...raw.Config };
    const call = {
      tokenProvider: async () => ({
        value: raw.Token,
        scheme: "Bearer",
        expiresAt: BigInt(raw.Expires),
      }),
    };
    const output =
      op === "encrypt"
        ? await g.encrypt(
            cfg,
            input,
            {
              Attributes: ["https://example.com/attr/attr1/value/value1"],
              SegmentSize: 2097152n,
              HasSegmentSize: true,
              SegmentHashAlgorithm: "GMAC",
            },
            call,
          )
        : (await g.decrypt(cfg, archive, call)).Payload;
    const elapsed = performance.now() - start;
    if (op === "decrypt") {
      if (
        output.length !== input.length ||
        !output.every((v, j) => v === input[j])
      )
        throw new Error("plaintext mismatch");
    } else await save(i, output);
    if (i >= 0) samples.push(elapsed);
  }
  return { samples_ms: samples, correct: true };
}
