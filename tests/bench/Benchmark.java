package consumer;
import io.opentdf.tdf3.TDF3;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;

/** End-to-end installed Java public API consumer. */
public final class Benchmark {
    @SuppressWarnings("unchecked")
    public static void main(String[] args) throws Exception {
        Path run = Path.of(args[0]);
        String op = args[1], size = args[2];
        if (!op.equals("e2e")) throw new IllegalArgumentException("e2e only");
        int n = Integer.parseInt(args[3]);
        int warmups = args.length > 4 ? Integer.parseInt(args[4]) : 1;
        int bulkWarmups = args.length > 5 ? Integer.parseInt(args[5]) : 0;
        if (n < 1 || warmups < 1 || bulkWarmups < 0) throw new IllegalArgumentException("invalid samples or warmups");
        Map<String,Object> raw = (Map<String,Object>)Json.parse(Files.readString(run.resolve("private.json")));
        byte[] input = Files.readAllBytes(run.resolve(size + ".input"));
        byte[] bulkInput = bulkWarmups == 0 || size.equals("50") ? input : Files.readAllBytes(run.resolve("50.input"));
        TDF3.Config cfg = Consumer.cfg((Map<String,Object>)raw.get("Config"));
        TDF3.CallOptions call = new TDF3.CallOptions();
        call.tokenProvider = request -> request.resolve(new TDF3.AccessToken(
            (String)raw.get("Token"), "Bearer", Long.parseLong(raw.get("Expires").toString()), ""));
        TDF3.EncryptOptions options = new TDF3.EncryptOptions();
        options.Attributes = new String[]{"https://example.com/attr/attr1/value/value1"};
        options.SegmentSize = 2 << 20;
        options.HasSegmentSize = true;
        options.SegmentHashAlgorithm = "GMAC";
        List<Double> samples = new ArrayList<>();
        List<Double> warmupHistory = new ArrayList<>();
        List<Double> bulkWarmupHistory = new ArrayList<>();
        for (int i = -bulkWarmups - warmups; i < n; i++) {
            byte[] pairInput = i < -warmups ? bulkInput : input;
            long start = System.nanoTime();
            byte[] archive = TDF3.encrypt(cfg, pairInput, options, call).completion().toCompletableFuture().get(1800, TimeUnit.SECONDS);
            byte[] output = TDF3.decrypt(cfg, archive, call).completion().toCompletableFuture().get(1800, TimeUnit.SECONDS).Payload();
            double elapsed = (System.nanoTime() - start) / 1e6;
            if (!Arrays.equals(pairInput, output)) throw new AssertionError("plaintext mismatch");
            if (i == -1 || i >= 0) Files.write(run.resolve("java-" + size + "-" + i + ".tdf"), archive);
            if (i >= 0) samples.add(elapsed);
            else if (i < -warmups) bulkWarmupHistory.add(elapsed);
            else warmupHistory.add(elapsed);
        }
        System.out.println("{\"samples_ms\":" + samples + ",\"warmup_ms\":" + warmupHistory + ",\"bulk_warmup_ms\":" + bulkWarmupHistory + ",\"warmup_count\":" + warmups + ",\"bulk_warmup_count\":" + bulkWarmups + ",\"correct\":true,\"kas_calls_expected\":" + (n + warmups + bulkWarmups) + "}");
    }
}
