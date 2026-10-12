package otdfgc;

import io.opentdf.tdf3.OpenTDFException;
import io.opentdf.tdf3.ReaderOptions;
import io.opentdf.tdf3.SDK;
import io.opentdf.tdf3.TDFOptions;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.nio.file.attribute.PosixFilePermissions;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.zip.ZipEntry;
import java.util.zip.ZipFile;

/**
 * otdfgc is the native Java CLI over the generated package. It implements the
 * xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc does:
 * one SDK per invocation, created with --host and the client credentials;
 * every flag this build cannot honour exits 2 as unsupported.
 */
public final class Main {
 static final int EXIT_SUCCESS = 0, EXIT_ERROR = 1, EXIT_USAGE = 2;
 static final String TARGET = "java";
 static final Duration TIMEOUT = Duration.ofSeconds(120);
 static final String USAGE = """
  Usage:
    otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
        [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1]
        [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
    otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
        [--no-verify-assertions] [--session-key-algorithm rsa:2048|ec:secp256r1]
        [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
    otdfgc inspect <in>
    otdfgc supports <feature>
    otdfgc --version [--json]
  """;

 // The otdfctl flag subset of the CLI contract; boolean flags take no value.
 static final Map<String, Boolean> COMMON = Map.of("out", false, "host", false, "with-client-creds", false, "with-client-creds-file", false, "dpop", false, "dpop-key", false, "tls-no-verify", true, "log-level", false);
 static final Map<String, Boolean> ENCRYPT_FLAGS = merge(COMMON, Map.of("attr", false, "mime-type", false, "with-assertions", false, "wrapping-key-algorithm", false, "target-mode", false, "policy-mode", false, "ecdsa-binding", true));
 static final Map<String, Boolean> DECRYPT_FLAGS = merge(COMMON, Map.of("with-assertion-verification-keys", false, "no-verify-assertions", true, "session-key-algorithm", false, "kas-allowlist", false));
 static final List<String> DPOP_ALGORITHMS = List.of("ES256", "ES384", "ES512", "RS256", "RS384", "RS512");
 // Rules naming the flags this build cannot honour, optionally only for values other than the allowed ones.
 static final List<Map.Entry<String, List<String>>> ENCRYPT_UNSUPPORTED = List.of(rule("with-assertions"), rule("ecdsa-binding"), rule("target-mode", "4.3.0"), rule("policy-mode", "encrypted"), rule("wrapping-key-algorithm", "rsa:2048", "ec:secp256r1"), Map.entry("dpop", DPOP_ALGORITHMS), rule("with-client-creds-file"));
 static final List<Map.Entry<String, List<String>>> DECRYPT_UNSUPPORTED = List.of(rule("with-assertion-verification-keys"), rule("session-key-algorithm", "rsa:2048", "ec:secp256r1"), Map.entry("dpop", DPOP_ALGORITHMS), rule("with-client-creds-file"));

 static Map<String, Boolean> merge(Map<String, Boolean> a, Map<String, Boolean> b) { Map<String, Boolean> m = new HashMap<>(a); m.putAll(b); return m; }
 static Map.Entry<String, List<String>> rule(String name, String... allowed) { return Map.entry(name, List.of(allowed)); }

 static final class Parsed {
  final Map<String, String> values = new HashMap<>();
  final Set<String> set = new java.util.HashSet<>();
  final List<String> positional = new ArrayList<>();
  String value(String name) { return values.getOrDefault(name, ""); }
  boolean flag(String name) { return set.contains(name); }
 }

 static Parsed parse(List<String> args, Map<String, Boolean> spec) {
  Parsed p = new Parsed();
  for (int i = 0; i < args.size(); i++) {
   String a = args.get(i);
   if (!a.startsWith("-") || a.equals("-")) { p.positional.add(a); continue; }
   String name = a.replaceFirst("^-+", ""), value = "";
   boolean inline = false;
   int eq = name.indexOf('=');
   if (eq >= 0) { value = name.substring(eq + 1); name = name.substring(0, eq); inline = true; }
   if (name.equals("o")) name = "out";
   Boolean bool = spec.get(name);
   if (bool == null) throw new IllegalArgumentException("unknown flag: " + a);
   if (bool) {
    if (inline && !value.equals("true") && !value.equals("false")) throw new IllegalArgumentException("flag --" + name + " takes no value");
    if (!inline || value.equals("true")) p.set.add(name);
    continue;
   }
   if (!inline) {
    if (i + 1 >= args.size()) throw new IllegalArgumentException("flag --" + name + " needs a value");
    value = args.get(++i);
   }
   p.values.put(name, value);
   p.set.add(name);
  }
  return p;
 }

 static String unsupported(Parsed p, List<Map.Entry<String, List<String>>> rules) {
  for (Map.Entry<String, List<String>> r : rules) {
   if (!p.flag(r.getKey())) continue;
   if (r.getValue().isEmpty()) return "--" + r.getKey();
   if (!r.getValue().contains(p.value(r.getKey()))) return "--" + r.getKey() + " " + p.value(r.getKey());
  }
  return "";
 }

 static List<String> splitList(String s) {
  List<String> out = new ArrayList<>();
  for (String part : s.split(",")) if (!part.trim().isEmpty()) out.add(part.trim());
  return out;
 }

 // describe renders an SDK failure: the reference SDK's wording plus otdfctl's "required obligations" suffix.
 static String describe(Exception e) {
  if (!(e instanceof OpenTDFException o)) return String.valueOf(e.getMessage());
  String msg = o.getMessage();
  if (!o.requiredObligations().isEmpty()) msg += "\nrequired obligations: [" + String.join(" ", o.requiredObligations()) + "]";
  return msg;
 }

 static String describeNew(Exception e, String host) {
  if (!(e instanceof OpenTDFException o)) return String.valueOf(e.getMessage());
  if (o.kind().equals("auth")) return "Failed to authenticate with flag-provided client credentials. (" + o.getMessage() + ")";
  if (o.getMessage().contains("platform unreachable or not responding") || o.kind().equals("transport") || o.kind().equals("deadline_exceeded"))
   return "Failed to connect to the platform. Is the platform accepting connections at '" + host + "'? (" + o.getMessage() + ")";
  return o.getMessage();
 }

 record Opened(SDK sdk, String host, int code) {}

 // Builds the SDK from --host, --with-client-creds and the DPoP and TLS flags; the SDK discovers everything else.
 static Opened newSDK(Parsed p, PrintStream stderr) throws IOException {
  String host = p.value("host").replaceAll("/+$", "");
  if (host.isEmpty()) { stderr.print("--host is required\n" + USAGE); return new Opened(null, host, EXIT_USAGE); }
  String creds = p.value("with-client-creds");
  if (creds.isEmpty()) { stderr.print("--with-client-creds is required\n" + USAGE); return new Opened(null, host, EXIT_USAGE); }
  Object parsed;
  try { parsed = Json.parse(creds); } catch (RuntimeException e) { parsed = null; }
  if (!(parsed instanceof Map<?, ?> c) || !(c.get("clientId") instanceof String id) || id.isEmpty() || !(c.get("clientSecret") instanceof String secret) || secret.isEmpty()) {
   stderr.println("--with-client-creds must be JSON with clientId and clientSecret");
   return new Opened(null, host, EXIT_USAGE);
  }
  SDK.Builder builder = SDK.builder(host).clientCredentials(id, secret);
  if (host.toLowerCase().startsWith("http://")) builder.insecurePlaintext(true);
  if (p.flag("tls-no-verify")) builder.insecureSkipVerify(true);
  if (!p.value("dpop").isEmpty()) builder.dpopAlgorithm(p.value("dpop"));
  if (!p.value("dpop-key").isEmpty()) {
   if (p.value("dpop").isEmpty()) { stderr.println("--dpop-key needs --dpop"); return new Opened(null, host, EXIT_USAGE); }
   try { builder.dpopKeyPem(Files.readString(Path.of(p.value("dpop-key")))); }
   catch (IOException e) { stderr.println("Failed to read DPoP key " + p.value("dpop-key") + ": " + e.getMessage()); return new Opened(null, host, EXIT_ERROR); }
  }
  try { return new Opened(builder.build(TIMEOUT), host, EXIT_SUCCESS); }
  catch (OpenTDFException e) { stderr.println(describeNew(e, host)); return new Opened(null, host, EXIT_ERROR); }
 }

 static void writeOut(String path, byte[] data, String permissions) throws IOException {
  Path tmp = Path.of(path + ".tmp");
  Files.write(tmp, data);
  try { Files.setPosixFilePermissions(tmp, PosixFilePermissions.fromString(permissions)); } catch (UnsupportedOperationException ignored) {}
  try { Files.move(tmp, Path.of(path), StandardCopyOption.REPLACE_EXISTING, StandardCopyOption.ATOMIC_MOVE); }
  catch (IOException e) { Files.deleteIfExists(tmp); throw e; }
 }

 static int version(List<String> args, PrintStream stdout, PrintStream stderr) {
  boolean asJson = false;
  for (String a : args) {
   if (a.equals("--json")) asJson = true;
   else { stderr.println("unknown flag: " + a); return EXIT_USAGE; }
  }
  String version, schema;
  List<String> features;
  try { version = SDK.version(); schema = SDK.tdfSpecVersion(); features = SDK.supportedFeatures(); }
  catch (OpenTDFException e) { stderr.println("Failed to read version: " + describe(e)); return EXIT_ERROR; }
  if (asJson) {
   Map<String, Object> info = new LinkedHashMap<>();
   info.put("sdk_version", version); info.put("schema_version", schema); info.put("supported_features", features); info.put("target", TARGET);
   stdout.println(Json.write(info, false));
   return EXIT_SUCCESS;
  }
  stdout.println("otdfgc (" + TARGET + ") sdk " + version + " tdf-spec " + schema + " features " + String.join(",", features));
  return EXIT_SUCCESS;
 }

 static int supports(List<String> args, PrintStream stdout, PrintStream stderr) {
  if (args.size() != 1 || args.get(0).startsWith("-")) { stderr.println("usage: otdfgc supports <feature>"); return EXIT_USAGE; }
  List<String> features;
  try { features = SDK.supportedFeatures(); }
  catch (OpenTDFException e) { stderr.println("Failed to read supported features: " + describe(e)); return EXIT_ERROR; }
  if (features.contains(args.get(0))) return EXIT_SUCCESS;
  stdout.println(args.get(0) + " unsupported");
  return EXIT_ERROR;
 }

 static byte[] readManifest(String path) throws IOException {
  try (ZipFile zip = new ZipFile(path)) {
   ZipEntry entry = zip.getEntry("0.manifest.json");
   if (entry == null) throw new IOException("manifest entry not found");
   try (var in = zip.getInputStream(entry)) {
    ByteArrayOutputStream out = new ByteArrayOutputStream();
    in.transferTo(out);
    return out.toByteArray();
   }
  }
 }

 static int inspect(List<String> args, PrintStream stdout, PrintStream stderr) {
  if (args.size() != 1 || args.get(0).startsWith("-")) { stderr.println("usage: otdfgc inspect <in>"); return EXIT_USAGE; }
  byte[] manifest;
  try { manifest = readManifest(args.get(0)); }
  catch (IOException | RuntimeException e) { stderr.println("not a valid TDF: " + e.getMessage()); return EXIT_ERROR; }
  try { stdout.println(Json.write(Json.parse(new String(manifest, StandardCharsets.UTF_8)), true)); }
  catch (RuntimeException e) { stderr.println("failed to inspect TDF: " + e.getMessage()); return EXIT_ERROR; }
  return EXIT_SUCCESS;
 }

 static int encrypt(List<String> args, PrintStream stderr) throws IOException {
  Parsed p;
  try { p = parse(args, ENCRYPT_FLAGS); } catch (IllegalArgumentException e) { stderr.print(e.getMessage() + "\n" + USAGE); return EXIT_USAGE; }
  String reason = unsupported(p, ENCRYPT_UNSUPPORTED);
  if (!reason.isEmpty()) { stderr.println("unsupported in this build: " + reason); return EXIT_USAGE; }
  if (p.positional.size() != 1 || p.value("out").isEmpty()) { stderr.print("encrypt needs exactly one input file and --out\n" + USAGE); return EXIT_USAGE; }
  byte[] input;
  try { input = Files.readAllBytes(Path.of(p.positional.get(0))); }
  catch (IOException e) { stderr.println("Failed to read " + p.positional.get(0) + ": " + e.getMessage()); return EXIT_ERROR; }
  Opened opened = newSDK(p, stderr);
  if (opened.code() != EXIT_SUCCESS) return opened.code();
  try (SDK sdk = opened.sdk()) {
   // As otdfctl: the platform's KAS is the default key access server and attributes drive autoconfigure.
   TDFOptions.Builder options = TDFOptions.builder().kasInformation(new TDFOptions.KASInfo(opened.host() + "/kas")).mimeType(p.value("mime-type"));
   if (!p.value("attr").isEmpty()) options.attributes(splitList(p.value("attr")));
   if (!p.value("wrapping-key-algorithm").isEmpty()) options.wrappingKeyAlgorithm(p.value("wrapping-key-algorithm"));
   if (!p.value("target-mode").isEmpty()) options.targetMode(p.value("target-mode"));
   byte[] data;
   try { data = sdk.encryptBytes(input, options.build(), TIMEOUT); }
   catch (OpenTDFException e) { stderr.println("Failed to encrypt: " + describe(e)); return EXIT_ERROR; }
   try { writeOut(p.value("out"), data, "rw-r--r--"); }
   catch (IOException e) { stderr.println("Failed to write encrypted file " + p.value("out") + ": " + e.getMessage()); return EXIT_ERROR; }
  }
  return EXIT_SUCCESS;
 }

 static int decrypt(List<String> args, PrintStream stderr) throws IOException {
  Parsed p;
  try { p = parse(args, DECRYPT_FLAGS); } catch (IllegalArgumentException e) { stderr.print(e.getMessage() + "\n" + USAGE); return EXIT_USAGE; }
  String reason = unsupported(p, DECRYPT_UNSUPPORTED);
  if (!reason.isEmpty()) { stderr.println("unsupported in this build: " + reason); return EXIT_USAGE; }
  if (p.positional.size() != 1 || p.value("out").isEmpty()) { stderr.print("decrypt needs exactly one input file and --out\n" + USAGE); return EXIT_USAGE; }
  byte[] input;
  try { input = Files.readAllBytes(Path.of(p.positional.get(0))); }
  catch (IOException e) { stderr.println("Failed to read file: " + e.getMessage()); return EXIT_ERROR; }
  Opened opened = newSDK(p, stderr);
  if (opened.code() != EXIT_SUCCESS) return opened.code();
  try (SDK sdk = opened.sdk()) {
   ReaderOptions.Builder options = ReaderOptions.builder().disableAssertionVerification(p.flag("no-verify-assertions"));
   if (!p.value("session-key-algorithm").isEmpty()) options.sessionKeyType(p.value("session-key-algorithm"));
   // otdfctl: "*" alone ignores the allowlist; any other list is explicit.
   List<String> allowlist = splitList(p.value("kas-allowlist"));
   if (allowlist.size() == 1 && allowlist.get(0).equals("*")) options.ignoreAllowlist(true);
   else if (!allowlist.isEmpty()) options.kasAllowlist(allowlist);
   byte[] plaintext;
   try { plaintext = sdk.decryptBytes(input, options.build(), TIMEOUT); }
   catch (OpenTDFException e) { stderr.println("Failed to decrypt file: " + describe(e)); return EXIT_ERROR; }
   try { writeOut(p.value("out"), plaintext, "rw-------"); }
   catch (IOException e) { stderr.println("Failed to write decrypted data to file: " + e.getMessage()); return EXIT_ERROR; }
  }
  return EXIT_SUCCESS;
 }

 static int run(List<String> args, PrintStream stdout, PrintStream stderr) throws IOException {
  if (args.isEmpty()) { stderr.print(USAGE); return EXIT_USAGE; }
  String command = args.get(0);
  List<String> rest = args.subList(1, args.size());
  switch (command) {
   case "--version", "version": return version(rest, stdout, stderr);
   case "supports": return supports(rest, stdout, stderr);
   case "inspect": return inspect(rest, stdout, stderr);
   case "encrypt": return encrypt(rest, stderr);
   case "decrypt": return decrypt(rest, stderr);
   case "help", "--help", "-h": stdout.print(USAGE); return EXIT_SUCCESS;
   default: stderr.print("unknown command: " + command + "\n" + USAGE); return EXIT_USAGE;
  }
 }

 public static void main(String[] args) throws IOException {
  PrintStream stdout = new PrintStream(System.out, true, StandardCharsets.UTF_8), stderr = new PrintStream(System.err, true, StandardCharsets.UTF_8);
  int code = run(Arrays.asList(args), stdout, stderr);
  stdout.flush();
  stderr.flush();
  System.exit(code);
 }
}

/** Minimal JSON reader and writer for the credentials, the version line and inspect. */
final class Json {
 private final String s;
 private int i;
 private Json(String s) { this.s = s; }

 static Object parse(String s) {
  Json j = new Json(s);
  Object v = j.value();
  j.ws();
  if (j.i != s.length()) throw new IllegalArgumentException("trailing data");
  return v;
 }
 private void ws() { while (i < s.length() && Character.isWhitespace(s.charAt(i))) i++; }
 private Object value() {
  ws();
  if (i >= s.length()) throw new IllegalArgumentException("unexpected end");
  char c = s.charAt(i);
  switch (c) {
   case '{': {
    i++;
    Map<String, Object> m = new LinkedHashMap<>();
    ws();
    if (s.charAt(i) == '}') { i++; return m; }
    for (;;) {
     ws();
     String k = str();
     ws();
     expect(':');
     m.put(k, value());
     ws();
     if (s.charAt(i) == ',') { i++; continue; }
     expect('}');
     return m;
    }
   }
   case '[': {
    i++;
    List<Object> l = new ArrayList<>();
    ws();
    if (s.charAt(i) == ']') { i++; return l; }
    for (;;) {
     l.add(value());
     ws();
     if (s.charAt(i) == ',') { i++; continue; }
     expect(']');
     return l;
    }
   }
   case '"': return str();
   case 't': expectWord("true"); return Boolean.TRUE;
   case 'f': expectWord("false"); return Boolean.FALSE;
   case 'n': expectWord("null"); return null;
   default: break;
  }
  int st = i;
  while (i < s.length() && "+-0123456789.eE".indexOf(s.charAt(i)) >= 0) i++;
  if (st == i) throw new IllegalArgumentException("unexpected character " + c);
  return new Number(s.substring(st, i));
 }
 record Number(String text) {}
 private void expectWord(String w) { if (!s.startsWith(w, i)) throw new IllegalArgumentException("unexpected token"); i += w.length(); }
 private void expect(char c) { if (i >= s.length() || s.charAt(i) != c) throw new IllegalArgumentException("expected " + c); i++; }
 private String str() {
  expect('"');
  StringBuilder b = new StringBuilder();
  for (;;) {
   if (i >= s.length()) throw new IllegalArgumentException("unterminated string");
   char c = s.charAt(i++);
   if (c == '"') return b.toString();
   if (c != '\\') { b.append(c); continue; }
   char e = s.charAt(i++);
   switch (e) {
    case 'n' -> b.append('\n');
    case 't' -> b.append('\t');
    case 'r' -> b.append('\r');
    case 'b' -> b.append('\b');
    case 'f' -> b.append('\f');
    case 'u' -> { b.append((char) Integer.parseInt(s.substring(i, i + 4), 16)); i += 4; }
    default -> b.append(e);
   }
  }
 }

 static String write(Object v, boolean pretty) {
  StringBuilder b = new StringBuilder();
  write(b, v, pretty, 0);
  return b.toString();
 }
 private static void indent(StringBuilder b, boolean pretty, int depth) { if (pretty) { b.append('\n'); b.append("  ".repeat(depth)); } }
 private static void write(StringBuilder b, Object v, boolean pretty, int depth) {
  if (v == null) b.append("null");
  else if (v instanceof String str) {
   b.append('"');
   for (int k = 0; k < str.length(); k++) {
    char c = str.charAt(k);
    switch (c) {
     case '"' -> b.append("\\\"");
     case '\\' -> b.append("\\\\");
     case '\n' -> b.append("\\n");
     case '\r' -> b.append("\\r");
     case '\t' -> b.append("\\t");
     default -> { if (c < 0x20) b.append(String.format("\\u%04x", (int) c)); else b.append(c); }
    }
   }
   b.append('"');
  } else if (v instanceof Boolean) b.append(v);
  else if (v instanceof Number n) b.append(n.text());
  else if (v instanceof Map<?, ?> m) {
   if (m.isEmpty()) { b.append("{}"); return; }
   b.append('{');
   boolean first = true;
   for (Map.Entry<?, ?> e : m.entrySet()) {
    if (!first) b.append(',');
    first = false;
    indent(b, pretty, depth + 1);
    write(b, e.getKey(), pretty, depth + 1);
    b.append(pretty ? ": " : ":");
    write(b, e.getValue(), pretty, depth + 1);
   }
   indent(b, pretty, depth);
   b.append('}');
  } else if (v instanceof List<?> l) {
   if (l.isEmpty()) { b.append("[]"); return; }
   b.append('[');
   for (int k = 0; k < l.size(); k++) {
    if (k > 0) b.append(',');
    indent(b, pretty, depth + 1);
    write(b, l.get(k), pretty, depth + 1);
   }
   indent(b, pretty, depth);
   b.append(']');
  } else b.append(v);
 }
}
