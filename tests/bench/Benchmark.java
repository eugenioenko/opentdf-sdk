package consumer;
import io.opentdf.tdf3.ReaderOptions;
import io.opentdf.tdf3.SDK;
import io.opentdf.tdf3.TDF3;
import io.opentdf.tdf3.TDFObject;
import io.opentdf.tdf3.TDFOptions;
import java.nio.file.*;
import java.util.*;

/** End-to-end installed Java handle-API consumer: one SDK per process, as the reference consumer. */
public final class Benchmark {
    static String text(Map<String,Object> o, String k) { return o.get(k) instanceof String s ? s : ""; }

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
        Map<String,Object> cfg = (Map<String,Object>)raw.get("Config");
        byte[] input = Files.readAllBytes(run.resolve(size + ".input"));
        byte[] bulkInput = bulkWarmups == 0 || size.equals("50") ? input : Files.readAllBytes(run.resolve("50.input"));
        String token = (String)raw.get("Token");
        long expires = Long.parseLong(raw.get("Expires").toString());
        String kasURL = text(cfg, "KASURL");
        try (SDK sdk = SDK.builder(text(cfg, "PlatformURL")).insecurePlaintext(true).tokenEndpoint(text(cfg, "TokenURL"))
                .tokenProvider(request -> request.resolve(new TDF3.AccessToken(token, "Bearer", expires, ""))).build()) {
            TDFOptions options = TDFOptions.builder().disableAutoconfigure(true)
                .kasInformation(new TDFOptions.KASInfo(kasURL, text(cfg, "KASPublicKeyPEM"), text(cfg, "KID"), "rsa:2048", false))
                .wrappingKeyAlgorithm("rsa:2048").segmentSize(2 << 20)
                .attributes("https://example.com/attr/attr1/value/value1").build();
            // loadTDF reuses the SDK's default RSA2048 session. Do not generate a replacement.
            ReaderOptions readerOptions = ReaderOptions.builder().kasAllowlist(List.of(kasURL)).build();
            List<Double> samples = new ArrayList<>();
            List<Double> warmupHistory = new ArrayList<>();
            List<Double> bulkHistory = new ArrayList<>();
            for (int i = -bulkWarmups - warmups; i < n; i++) {
                byte[] pairInput = i < -warmups ? bulkInput : input;
                long start = System.nanoTime();
                byte[] archive;
                try (TDFObject object = sdk.createTDF(pairInput, options)) { archive = object.bytes(); }
                byte[] output = sdk.decryptBytes(archive, readerOptions);
                double elapsed = (System.nanoTime() - start) / 1e6;
                if (!Arrays.equals(output, pairInput)) throw new IllegalStateException("plaintext mismatch");
                if (i == -1 || i >= 0) Files.write(run.resolve("java-" + size + "-" + i + ".tdf"), archive);
                if (i >= 0) samples.add(elapsed);
                else if (i < -warmups) bulkHistory.add(elapsed);
                else warmupHistory.add(elapsed);
            }
            Map<String,Object> result = new LinkedHashMap<>();
            result.put("samples_ms", samples);
            result.put("warmup_ms", warmupHistory);
            result.put("bulk_warmup_ms", bulkHistory);
            result.put("warmup_count", warmups);
            result.put("bulk_warmup_count", bulkWarmups);
            result.put("correct", true);
            result.put("kas_calls_expected", n + warmups + bulkWarmups);
            System.out.println(Json.write(result));
        }
    }
}
