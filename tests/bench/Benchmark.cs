using System;
using System.IO;
using System.Text.Json;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using OpenTDF;

// End-to-end installed C# handle-API consumer: one SDK per process, as the reference consumer.
static class Benchmark {
    static string Text(JsonElement value, string key) => value.TryGetProperty(key, out var result) && result.ValueKind == JsonValueKind.String ? result.GetString() : "";

    static async Task Main(string[] args) {
        string run = args[0], op = args[1], size = args[2];
        if (op != "e2e") throw new ArgumentException("e2e only");
        int n = int.Parse(args[3]);
        int warmups = args.Length > 4 ? int.Parse(args[4]) : 1;
        int bulkWarmups = args.Length > 5 ? int.Parse(args[5]) : 0;
        if (n < 1 || warmups < 1 || bulkWarmups < 0) throw new ArgumentException("invalid samples or warmups");
        var raw = JsonDocument.Parse(File.ReadAllText(Path.Combine(run, "private.json"))).RootElement;
        var cfg = raw.GetProperty("Config");
        byte[] input = File.ReadAllBytes(Path.Combine(run, size + ".input"));
        byte[] bulkInput = bulkWarmups == 0 || size == "50" ? input : File.ReadAllBytes(Path.Combine(run, "50.input"));
        string token = raw.GetProperty("Token").GetString();
        long expires = raw.GetProperty("Expires").GetInt64();
        string kasUrl = Text(cfg, "KASURL");
        await using var sdk = await Sdk.CreateAsync(Text(cfg, "PlatformURL"), new Options {
            InsecurePlaintext = true, TokenEndpoint = Text(cfg, "TokenURL"),
            TokenProvider = TDF3.FromTask(request => Task.FromResult(new TDF3.AccessToken(token, "Bearer", expires)))
        });
        var options = new TdfOptions {
            DisableAutoconfigure = true,
            KasInformation = new[] { new KasInfo(kasUrl) { PublicKey = Text(cfg, "KASPublicKeyPEM"), Kid = Text(cfg, "KID"), Algorithm = "rsa:2048" } },
            WrappingKeyAlgorithm = "rsa:2048", SegmentSize = 2 << 20,
            Attributes = new[] { "https://example.com/attr/attr1/value/value1" }
        };
        // LoadTdf reuses the SDK's default RSA2048 session. Do not generate a replacement.
        var readerOptions = new ReaderOptions { KasAllowlist = new List<string> { kasUrl } };
        var samples = new List<double>();
        var warmupHistory = new List<double>();
        var bulkHistory = new List<double>();
        for (int i = -bulkWarmups - warmups; i < n; i++) {
            byte[] pairInput = i < -warmups ? bulkInput : input;
            long start = Stopwatch.GetTimestamp();
            byte[] archive = await sdk.EncryptBytesAsync(pairInput, options);
            byte[] output = await sdk.DecryptBytesAsync(archive, readerOptions);
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
