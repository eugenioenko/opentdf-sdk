using System;
using System.IO;
using System.Text.Json;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using OpenTDF;

static class Benchmark {
    static void Main(string[] args) {
        string run = args[0], op = args[1], size = args[2];
        if (op != "e2e") throw new ArgumentException("e2e only");
        int n = int.Parse(args[3]);
        int warmups = args.Length > 4 ? int.Parse(args[4]) : 1;
        int bulkWarmups = args.Length > 5 ? int.Parse(args[5]) : 0;
        if (n < 1 || warmups < 1 || bulkWarmups < 0) throw new ArgumentException("invalid samples or warmups");
        var raw = JsonDocument.Parse(File.ReadAllText(Path.Combine(run, "private.json"))).RootElement;
        byte[] input = File.ReadAllBytes(Path.Combine(run, size + ".input"));
        byte[] bulkInput = bulkWarmups == 0 || size == "50" ? input : File.ReadAllBytes(Path.Combine(run, "50.input"));
        var cfg = JsonSerializer.Deserialize<TDF3.Config>(raw.GetProperty("Config").GetRawText(), new JsonSerializerOptions { IncludeFields = true });
        var call = new TDF3.CallOptions {
            Provider = request => request.Resolve(new TDF3.AccessToken(raw.GetProperty("Token").GetString(), "Bearer", raw.GetProperty("Expires").GetInt64()))
        };
        var options = new TDF3.EncryptOptions {
            Attributes = new[] { "https://example.com/attr/attr1/value/value1" },
            SegmentSize = 2 << 20, HasSegmentSize = true, SegmentHashAlgorithm = "GMAC"
        };
        var samples = new List<double>();
        var warmupHistory = new List<double>();
        var bulkHistory = new List<double>();
        for (int i = -bulkWarmups - warmups; i < n; i++) {
            byte[] pairInput = i < -warmups ? bulkInput : input;
            long start = Stopwatch.GetTimestamp();
            byte[] archive = TDF3.Encrypt(cfg, pairInput, options, call).Completion.GetAwaiter().GetResult();
            byte[] output = TDF3.Decrypt(cfg, archive, call).Completion.GetAwaiter().GetResult().Payload;
            double elapsed = Stopwatch.GetElapsedTime(start).TotalMilliseconds;
            if (!output.SequenceEqual(pairInput)) throw new Exception("plaintext mismatch");
            if (i == -1 || i >= 0) File.WriteAllBytes(Path.Combine(run, $"csharp-{size}-{i}.tdf"), archive);
            if (i >= 0) samples.Add(elapsed);
            else if (i < -warmups) bulkHistory.Add(elapsed);
            else warmupHistory.Add(elapsed);
        }
        Console.WriteLine(JsonSerializer.Serialize(new { samples_ms = samples, warmup_ms = warmupHistory, bulk_warmup_ms = bulkHistory, warmup_count = warmups, bulk_warmup_count = bulkWarmups, correct = true, kas_calls_expected = n + warmups + bulkWarmups }));
    }
}
