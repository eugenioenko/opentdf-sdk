// otdfgc is the native Swift CLI over the generated package. It implements
// the xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc
// does: one SDK per invocation, created with --host and the client
// credentials; every flag this build cannot honour exits 2 as unsupported.
import Foundation
import OpenTDFTDF3

let exitSuccess: Int32 = 0, exitError: Int32 = 1, exitUsage: Int32 = 2
let target = "swift"
let timeoutMillis: Int64 = 120_000
let usage = """
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

  """.split(separator: "\n", omittingEmptySubsequences: false).map { $0.hasPrefix("  ") ? String($0.dropFirst(2)) : String($0) }.joined(separator: "\n")

// The otdfctl flag subset of the CLI contract; boolean flags take no value.
let commonFlags: [String: Bool] = ["out": false, "host": false, "with-client-creds": false, "with-client-creds-file": false, "dpop": false, "dpop-key": false, "tls-no-verify": true, "log-level": false]
let encryptFlags = commonFlags.merging(["attr": false, "mime-type": false, "with-assertions": false, "wrapping-key-algorithm": false, "target-mode": false, "policy-mode": false, "ecdsa-binding": true]) { $1 }
let decryptFlags = commonFlags.merging(["with-assertion-verification-keys": false, "no-verify-assertions": true, "session-key-algorithm": false, "kas-allowlist": false]) { $1 }
let dpopAlgorithms = ["ES256", "ES384", "ES512", "RS256", "RS384", "RS512"]
// Rules naming the flags this build cannot honour, optionally only for values other than the allowed ones.
let encryptUnsupported: [(String, [String]?)] = [("with-assertions", nil), ("ecdsa-binding", nil), ("target-mode", ["4.3.0"]), ("policy-mode", ["encrypted"]), ("wrapping-key-algorithm", ["rsa:2048", "ec:secp256r1"]), ("dpop", dpopAlgorithms), ("with-client-creds-file", nil)]
let decryptUnsupported: [(String, [String]?)] = [("with-assertion-verification-keys", nil), ("session-key-algorithm", ["rsa:2048", "ec:secp256r1"]), ("dpop", dpopAlgorithms), ("with-client-creds-file", nil)]

struct Parsed {
  var values: [String: String] = [:]
  var set: Set<String> = []
  var positional: [String] = []
  func value(_ name: String) -> String { values[name] ?? "" }
  func flag(_ name: String) -> Bool { set.contains(name) }
}
struct UsageError: Error { let message: String }

func parse(_ args: [String], _ spec: [String: Bool]) throws -> Parsed {
  var p = Parsed()
  var i = 0
  while i < args.count {
    let a = args[i]
    if !a.hasPrefix("-") || a == "-" { p.positional.append(a); i += 1; continue }
    var name = String(a.drop { $0 == "-" })
    var value = ""
    var inline = false
    if let eq = name.firstIndex(of: "=") { value = String(name[name.index(after: eq)...]); name = String(name[..<eq]); inline = true }
    if name == "o" { name = "out" }
    guard let isBool = spec[name] else { throw UsageError(message: "unknown flag: \(a)") }
    if isBool {
      if inline && value != "true" && value != "false" { throw UsageError(message: "flag --\(name) takes no value") }
      if !inline || value == "true" { p.set.insert(name) }
      i += 1
      continue
    }
    if !inline {
      guard i + 1 < args.count else { throw UsageError(message: "flag --\(name) needs a value") }
      i += 1
      value = args[i]
    }
    p.values[name] = value
    p.set.insert(name)
    i += 1
  }
  return p
}
func unsupported(_ p: Parsed, _ rules: [(String, [String]?)]) -> String {
  for (name, allowed) in rules where p.flag(name) {
    guard let allowed = allowed else { return "--\(name)" }
    if !allowed.contains(p.value(name)) { return "--\(name) \(p.value(name))" }
  }
  return ""
}
func splitList(_ s: String) -> [String] { s.split(separator: ",").map { $0.trimmingCharacters(in: .whitespaces) }.filter { !$0.isEmpty } }
func stderr(_ s: String) { FileHandle.standardError.write(Data(s.utf8)) }
func stdout(_ s: String) { FileHandle.standardOutput.write(Data(s.utf8)) }

// describe renders an SDK failure: the reference SDK's wording plus otdfctl's "required obligations" suffix.
func describe(_ e: Error) -> String {
  guard let o = e as? OpenTDFError else { return "\(e)" }
  var msg = o.message
  if !o.requiredObligations.isEmpty { msg += "\nrequired obligations: [\(o.requiredObligations.joined(separator: " "))]" }
  return msg
}
func describeNew(_ e: Error, _ host: String) -> String {
  guard let o = e as? OpenTDFError else { return "\(e)" }
  if o.kind == .auth { return "Failed to authenticate with flag-provided client credentials. (\(o.message))" }
  if o.message.contains("platform unreachable or not responding") || o.kind == .transport || o.kind == .deadlineExceeded {
    return "Failed to connect to the platform. Is the platform accepting connections at '\(host)'? (\(o.message))"
  }
  return o.message
}
func jsonText(_ value: Any, pretty: Bool) -> String {
  let options: JSONSerialization.WritingOptions = pretty ? [.prettyPrinted, .withoutEscapingSlashes, .fragmentsAllowed] : [.withoutEscapingSlashes, .fragmentsAllowed]
  return (try? JSONSerialization.data(withJSONObject: value, options: options)).map { String(decoding: $0, as: UTF8.self) } ?? "null"
}

// Builds the SDK from --host, --with-client-creds and the DPoP and TLS flags; the SDK discovers everything else.
func newSDK(_ p: Parsed) async -> (SDK?, String, Int32) {
  var host = p.value("host")
  while host.hasSuffix("/") { host.removeLast() }
  if host.isEmpty { stderr("--host is required\n" + usage); return (nil, host, exitUsage) }
  let creds = p.value("with-client-creds")
  if creds.isEmpty { stderr("--with-client-creds is required\n" + usage); return (nil, host, exitUsage) }
  let parsed = (try? JSONSerialization.jsonObject(with: Data(creds.utf8))) as? [String: Any]
  guard let id = parsed?["clientId"] as? String, !id.isEmpty, let secret = parsed?["clientSecret"] as? String, !secret.isEmpty else {
    stderr("--with-client-creds must be JSON with clientId and clientSecret\n")
    return (nil, host, exitUsage)
  }
  var options = Options(clientId: id, clientSecret: secret, insecurePlaintext: host.lowercased().hasPrefix("http://"), insecureSkipVerify: p.flag("tls-no-verify"), timeoutMillis: timeoutMillis)
  options.dpopAlgorithm = p.value("dpop")
  if !p.value("dpop-key").isEmpty {
    if p.value("dpop").isEmpty { stderr("--dpop-key needs --dpop\n"); return (nil, host, exitUsage) }
    do { options.dpopKeyPem = try String(contentsOfFile: p.value("dpop-key"), encoding: .utf8) }
    catch { stderr("Failed to read DPoP key \(p.value("dpop-key")): \(error.localizedDescription)\n"); return (nil, host, exitError) }
  }
  do { return (try await SDK(host, options), host, exitSuccess) }
  catch { stderr(describeNew(error, host) + "\n"); return (nil, host, exitError) }
}
func writeOut(_ path: String, _ data: Data, _ mode: Int) throws {
  let tmp = path + ".tmp"
  try data.write(to: URL(fileURLWithPath: tmp))
  try FileManager.default.setAttributes([.posixPermissions: mode], ofItemAtPath: tmp)
  do {
    if FileManager.default.fileExists(atPath: path) { _ = try FileManager.default.replaceItemAt(URL(fileURLWithPath: path), withItemAt: URL(fileURLWithPath: tmp)) }
    else { try FileManager.default.moveItem(atPath: tmp, toPath: path) }
  } catch { try? FileManager.default.removeItem(atPath: tmp); throw error }
}

func version(_ args: [String]) -> Int32 {
  var asJson = false
  for a in args {
    if a == "--json" { asJson = true } else { stderr("unknown flag: \(a)\n"); return exitUsage }
  }
  let version: String, schema: String, features: [String]
  do { version = try SDK.version(); schema = try SDK.tdfSpecVersion(); features = try SDK.supportedFeatures() }
  catch { stderr("Failed to read version: \(describe(error))\n"); return exitError }
  if asJson {
    stdout("{\"sdk_version\":\(jsonText(version, pretty: false)),\"schema_version\":\(jsonText(schema, pretty: false)),\"supported_features\":\(jsonText(features, pretty: false)),\"target\":\"\(target)\"}\n")
    return exitSuccess
  }
  stdout("otdfgc (\(target)) sdk \(version) tdf-spec \(schema) features \(features.joined(separator: ","))\n")
  return exitSuccess
}
func supports(_ args: [String]) -> Int32 {
  guard args.count == 1, !args[0].hasPrefix("-") else { stderr("usage: otdfgc supports <feature>\n"); return exitUsage }
  do {
    if try SDK.supportedFeatures().contains(args[0]) { return exitSuccess }
  } catch { stderr("Failed to read supported features: \(describe(error))\n"); return exitError }
  stdout("\(args[0]) unsupported\n")
  return exitError
}
// inspect reads the manifest through the handle API: an SDK with a provided
// platform configuration needs no platform, and the loaded reader's manifest
// is printed keyed by the reference's JSON tags.
func inspect(_ args: [String]) async -> Int32 {
  guard args.count == 1, !args[0].hasPrefix("-") else { stderr("usage: otdfgc inspect <in>\n"); return exitUsage }
  let data: Data
  do { data = try Data(contentsOf: URL(fileURLWithPath: args[0])) }
  catch { stderr("not a valid TDF: \(error.localizedDescription)\n"); return exitError }
  let m: Manifest
  do {
    let sdk = try await SDK("http://localhost", Options(platformConfiguration: ["platform_issuer": "http://localhost"], insecurePlaintext: true))
    let reader = try await sdk.loadTDF(data, ReaderOptions(ignoreAllowlist: true))
    m = try await reader.manifest()
    try reader.close()
    try sdk.close()
  } catch { stderr("not a valid TDF: \(describe(error))\n"); return exitError }
  let e = m.encryptionInformation, i = e.integrityInformation
  let json: [String: Any] = [
    "encryptionInformation": [
      "type": e.type, "policy": e.policy,
      "keyAccess": e.keyAccess.map { ["type": $0.type, "url": $0.url, "protocol": $0.`protocol`, "wrappedKey": $0.wrappedKey, "policyBinding": ["alg": $0.policyBinding.alg, "hash": $0.policyBinding.hash], "encryptedMetadata": $0.encryptedMetadata, "kid": $0.kid, "sid": $0.sid, "schemaVersion": $0.schemaVersion, "ephemeralPublicKey": $0.ephemeralPublicKey] as [String: Any] },
      "method": ["algorithm": e.method.algorithm, "iv": e.method.iv, "isStreamable": e.method.isStreamable] as [String: Any],
      "integrityInformation": [
        "rootSignature": ["alg": i.rootSignature.alg, "sig": i.rootSignature.sig], "segmentHashAlg": i.segmentHashAlg,
        "segmentSizeDefault": i.segmentSizeDefault, "encryptedSegmentSizeDefault": i.encryptedSegmentSizeDefault,
        "segments": i.segments.map { ["hash": $0.hash, "segmentSize": $0.segmentSize, "encryptedSegmentSize": $0.encryptedSegmentSize] as [String: Any] },
      ] as [String: Any],
    ] as [String: Any],
    "payload": ["type": m.payload.type, "url": m.payload.url, "protocol": m.payload.`protocol`, "mimeType": m.payload.mimeType, "isEncrypted": m.payload.isEncrypted] as [String: Any],
    "assertions": m.assertions.map { ["id": $0.id, "type": $0.type, "scope": $0.scope, "appliesToState": $0.appliesToState, "statement": ["format": $0.statement.format, "schema": $0.statement.schema, "value": $0.statement.value], "binding": ["method": $0.binding.method, "signature": $0.binding.signature]] as [String: Any] },
    "schemaVersion": m.schemaVersion,
  ]
  stdout(jsonText(json, pretty: true) + "\n")
  return exitSuccess
}
func encrypt(_ args: [String]) async -> Int32 {
  let p: Parsed
  do { p = try parse(args, encryptFlags) } catch let e as UsageError { stderr(e.message + "\n" + usage); return exitUsage } catch { return exitUsage }
  let reason = unsupported(p, encryptUnsupported)
  if !reason.isEmpty { stderr("unsupported in this build: \(reason)\n"); return exitUsage }
  guard p.positional.count == 1, !p.value("out").isEmpty else { stderr("encrypt needs exactly one input file and --out\n" + usage); return exitUsage }
  let input: Data
  do { input = try Data(contentsOf: URL(fileURLWithPath: p.positional[0])) }
  catch { stderr("Failed to read \(p.positional[0]): \(error.localizedDescription)\n"); return exitError }
  let (sdk, host, code) = await newSDK(p)
  guard let sdk = sdk else { return code }
  defer { try? sdk.close() }
  // As otdfctl: the platform's KAS is the default key access server and attributes drive autoconfigure.
  let options = TDFOptions(attributes: splitList(p.value("attr")), kasInformation: [KASInfo(url: host + "/kas")], wrappingKeyAlgorithm: p.value("wrapping-key-algorithm"), mimeType: p.value("mime-type"), targetMode: p.value("target-mode"))
  let data: Data
  do { data = try await sdk.encryptBytes(input, options) }
  catch { stderr("Failed to encrypt: \(describe(error))\n"); return exitError }
  do { try writeOut(p.value("out"), data, 0o644) }
  catch { stderr("Failed to write encrypted file \(p.value("out")): \(error.localizedDescription)\n"); return exitError }
  return exitSuccess
}
func decrypt(_ args: [String]) async -> Int32 {
  let p: Parsed
  do { p = try parse(args, decryptFlags) } catch let e as UsageError { stderr(e.message + "\n" + usage); return exitUsage } catch { return exitUsage }
  let reason = unsupported(p, decryptUnsupported)
  if !reason.isEmpty { stderr("unsupported in this build: \(reason)\n"); return exitUsage }
  guard p.positional.count == 1, !p.value("out").isEmpty else { stderr("decrypt needs exactly one input file and --out\n" + usage); return exitUsage }
  let input: Data
  do { input = try Data(contentsOf: URL(fileURLWithPath: p.positional[0])) }
  catch { stderr("Failed to read file: \(error.localizedDescription)\n"); return exitError }
  let (sdk, _, code) = await newSDK(p)
  guard let sdk = sdk else { return code }
  defer { try? sdk.close() }
  // otdfctl: "*" alone ignores the allowlist; any other list is explicit.
  let allowlist = splitList(p.value("kas-allowlist"))
  let ignore = allowlist.count == 1 && allowlist[0] == "*"
  let options = ReaderOptions(disableAssertionVerification: p.flag("no-verify-assertions"), kasAllowlist: !ignore && !allowlist.isEmpty ? allowlist : nil, ignoreAllowlist: ignore, sessionKeyType: p.value("session-key-algorithm"))
  let plaintext: Data
  do { plaintext = try await sdk.decryptBytes(input, options) }
  catch { stderr("Failed to decrypt file: \(describe(error))\n"); return exitError }
  do { try writeOut(p.value("out"), plaintext, 0o600) }
  catch { stderr("Failed to write decrypted data to file: \(error.localizedDescription)\n"); return exitError }
  return exitSuccess
}

@main
struct Main {
  static func main() async {
    let args = Array(CommandLine.arguments.dropFirst())
    guard let command = args.first else { stderr(usage); exit(exitUsage) }
    let rest = Array(args.dropFirst())
    let code: Int32
    switch command {
    case "--version", "version": code = version(rest)
    case "supports": code = supports(rest)
    case "inspect": code = await inspect(rest)
    case "encrypt": code = await encrypt(rest)
    case "decrypt": code = await decrypt(rest)
    case "help", "--help", "-h": stdout(usage); code = exitSuccess
    default: stderr("unknown command: \(command)\n" + usage); code = exitUsage
    }
    exit(code)
  }
}
