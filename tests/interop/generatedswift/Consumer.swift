import Foundation
import GoalchemyNative
import OpenTDFTDF3

#if canImport(FoundationNetworking)
  import FoundationNetworking
#endif

// Independent importing consumer of the handle API: one SDK per run,
// createTDF/decryptBytes, loadTDF readers, 100 round trips, and the controlled
// checks of tests/interop/delivery/controlled_platform.py. The host token
// source signs its DPoP proof with the native C ABI, independently of the
// shared source's OAuth/DPoP implementation.
private final class HostProof {
  let key: UnsafeMutableRawPointer
  let rsa: Bool
  let jwk: [String: String]
  let thumbprint: String
  static func b64(_ value: Data) -> String {
    value.base64EncodedString().replacingOccurrences(of: "+", with: "-")
      .replacingOccurrences(of: "/", with: "_").replacingOccurrences(of: "=", with: "")
  }
  init(_ pem: String, mismatch: Bool) throws {
    let bytes = Array(pem.utf8)
    let imported = mismatch ? gcn_key_generate(256) : bytes.withUnsafeBufferPointer { gcn_key_import($0.baseAddress, $0.count) }
    guard let imported = imported else { throw NSError(domain: "proof", code: 1) }
    key = imported
    rsa = gcn_key_kind(imported) == 1
    func component(_ index: Int32) throws -> String {
      var out = [UInt8](repeating: 0, count: 512)
      var length = out.count
      let status = out.withUnsafeMutableBufferPointer { gcn_key_component(imported, index, $0.baseAddress, &length) }
      guard status == 1 else { throw NSError(domain: "proof", code: 2) }
      return Self.b64(Data(out.prefix(length)))
    }
    jwk = rsa ? ["kty": "RSA", "n": try component(0), "e": try component(1)] : ["kty": "EC", "crv": "P-256", "x": try component(2), "y": try component(3)]
    let data = try JSONSerialization.data(withJSONObject: jwk, options: [.sortedKeys, .withoutEscapingSlashes])
    var digest = [UInt8](repeating: 0, count: 32)
    let status = data.withUnsafeBytes { source in digest.withUnsafeMutableBufferPointer { gcn_sha256(source.bindMemory(to: UInt8.self).baseAddress, data.count, $0.baseAddress) } }
    guard status == 1 else { throw NSError(domain: "proof", code: 3) }
    thumbprint = Self.b64(Data(digest))
  }
  deinit { gcn_key_release(key) }
  func sign(_ endpoint: String, _ nonce: String) throws -> String {
    let header: [String: Any] = ["typ": "dpop+jwt", "alg": rsa ? "RS256" : "ES256", "jwk": jwk]
    var claims: [String: Any] = ["htu": endpoint, "htm": "POST", "iat": Int64(Date().timeIntervalSince1970), "jti": UUID().uuidString]
    if !nonce.isEmpty { claims["nonce"] = nonce }
    let text = Self.b64(try JSONSerialization.data(withJSONObject: header, options: [.sortedKeys, .withoutEscapingSlashes])) + "." + Self.b64(try JSONSerialization.data(withJSONObject: claims, options: [.sortedKeys, .withoutEscapingSlashes]))
    let data = Data(text.utf8)
    var signature = [UInt8](repeating: 0, count: 512)
    var length = signature.count
    let status = data.withUnsafeBytes { source in signature.withUnsafeMutableBufferPointer { gcn_sign(rsa ? 1 : 2, 256, key, source.bindMemory(to: UInt8.self).baseAddress, data.count, $0.baseAddress, &length) } }
    guard status == 1 else { throw NSError(domain: "proof", code: 4) }
    return text + "." + Self.b64(Data(signature.prefix(length)))
  }
}

struct Failure: Error { let label: String }
func check(_ ok: Bool, _ label: String) throws { if !ok { throw Failure(label: label) } }
let attribute = "https://example.com/attr/attr1/value/value1"

// The host token source: native OAuth client credentials with a DPoP proof when the profile asks for one.
func tokenProvider(_ raw: [String: Any], _ name: String) -> TokenProvider {
  return { request, complete in
    if name == "provider-reject" { complete(.failure(NSError(domain: "fixture", code: 1))); return nil }
    if name == "invalid-token" || name == "expired-token" {
      complete(.success(AccessToken(value: "invalid", scheme: "Bearer", expiresAt: Int64(Date().timeIntervalSince1970) + (name == "expired-token" ? -1 : 300))))
      return nil
    }
    let dpop = raw["DPoP"] as? Bool ?? false
    let endpoint = (raw["IssuerURL"] as? String ?? "") + "/protocol/openid-connect/token"
    let proof: HostProof?
    do { proof = dpop ? try HostProof(raw["AuthPrivateKeyPEM"] as? String ?? "", mismatch: name == "mismatched-provider") : nil } catch { complete(.failure(error)); return nil }
    var activeTask: URLSessionDataTask?
    let taskLock = NSLock()
    func attempt(_ nonce: String, _ count: Int) {
      if request.cancellation.isCanceled { complete(.failure(NSError(domain: "canceled", code: 1))); return }
      var message = URLRequest(url: URL(string: endpoint)!)
      message.httpMethod = "POST"
      message.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")
      message.httpBody = Data("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret".utf8)
      do { if let proof = proof { message.setValue(try proof.sign(endpoint, nonce), forHTTPHeaderField: "DPoP") } } catch { complete(.failure(error)); return }
      let task = URLSession.shared.dataTask(with: message) { data, response, error in
        do {
          if let error = error { throw error }
          if let response = response as? HTTPURLResponse, [400, 401].contains(response.statusCode), count < 3, let next = response.value(forHTTPHeaderField: "DPoP-Nonce"), !next.isEmpty, next != nonce { attempt(next, count + 1); return }
          guard (response as? HTTPURLResponse)?.statusCode == 200, let data = data, let value = try JSONSerialization.jsonObject(with: data) as? [String: Any], let token = value["access_token"] as? String, let seconds = value["expires_in"] as? NSNumber
          else { throw NSError(domain: "provider", code: 2) }
          complete(.success(AccessToken(value: token, scheme: proof == nil ? "Bearer" : "DPoP", expiresAt: Int64(Date().timeIntervalSince1970) + seconds.int64Value, confirmationJKT: proof?.thumbprint ?? "")))
        } catch { complete(.failure(error)) }
      }
      taskLock.lock(); activeTask = task; taskLock.unlock()
      task.resume()
    }
    attempt("", 1)
    return { taskLock.lock(); let task = activeTask; taskLock.unlock(); task?.cancel() }
  }
}

// config.json carries the façade Config fields the delivery scripts write; they map onto the handle API's options as the Go consumer maps them.
func optionsOf(_ raw: [String: Any], _ name: String, credentials: Bool = true) -> Options {
  func text(_ key: String) -> String { raw[key] as? String ?? "" }
  var o = Options(insecurePlaintext: raw["AllowHTTP"] as? Bool ?? false)
  o.tokenEndpoint = text("TokenURL")
  if credentials { o.clientId = text("ClientID"); o.clientSecret = text("ClientSecret") }
  if !text("TokenProviderName").isEmpty { o.tokenProvider = tokenProvider(raw, name) }
  o.timeoutMillis = (raw["TimeoutMillis"] as? NSNumber)?.int64Value ?? 0
  if raw["DPoP"] as? Bool ?? false { o.dpopAlgorithm = text("AuthAlgorithm").isEmpty ? "ES256" : text("AuthAlgorithm") }
  o.dpopKeyPem = text("AuthPrivateKeyPEM")
  return o
}
func kasInformation(_ raw: [String: Any]) -> KASInfo {
  func text(_ key: String) -> String { raw[key] as? String ?? "" }
  var platform = text("PlatformURL"); while platform.hasSuffix("/") { platform.removeLast() }
  return KASInfo(url: text("KASURL").isEmpty ? platform + "/kas" : text("KASURL"), publicKey: text("KASPublicKeyPEM"), kid: text("KID"), algorithm: text("KASAlgorithm"))
}
func tdfOptions(_ raw: [String: Any]) -> TDFOptions {
  TDFOptions(attributes: [attribute], disableAutoconfigure: true, kasInformation: [kasInformation(raw)], wrappingKeyAlgorithm: raw["KASAlgorithm"] as? String ?? "", segmentSize: 16384)
}
func readerOptions(_ raw: [String: Any]) -> ReaderOptions {
  var allow = [raw["PlatformURL"] as? String ?? ""]
  if let kas = raw["KASURL"] as? String, !kas.isEmpty { allow.append(kas) }
  for route in raw["AllowedKAS"] as? [[String: Any]] ?? [] { allow.append(route["URL"] as? String ?? "") }
  return ReaderOptions(kasAllowlist: allow, sessionKeyType: raw["SessionAlgorithm"] as? String ?? "")
}
func report(_ e: Error) -> [String: Any] {
  guard let o = e as? OpenTDFError else { return ["untyped": "\(e)"] }
  return ["kind": o.kind.name, "op": o.op, "httpStatus": o.httpStatus, "connectCode": o.connectCode, "message": o.message]
}
func expectFailure<T>(_ label: String, _ call: () async throws -> T) async throws -> [String: Any] {
  do { _ = try await call() } catch let e as OpenTDFError { return report(e) } catch { throw Failure(label: label + " untyped: \(error)") }
  throw Failure(label: label + " returned successful output")
}
func writeJSON(_ run: URL, _ file: String, _ value: [String: Any]) throws {
  try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]).write(to: run.appendingPathComponent(file))
}

/// 100 encrypt and decrypt calls on one SDK, independent inputs and results, every handle closed; then the SDK is closed and later calls fail.
func repeatRounds(_ sdk: SDK, _ raw: [String: Any], _ name: String) async throws {
  let original = Data([0, 255, 128, 1])
  var archive = Data()
  for i in 0..<100 {
    var input = original; input.append(UInt8(i))
    var options = tdfOptions(raw); options.metadata = "round \(i)"; options.mimeType = "application/日本語"
    let object = try await sdk.createTDF(input, options)
    try check(try await object.size() == Int64(object.bytes.count), "size")
    try check(try await object.manifest().payload.mimeType == "application/日本語", "UTF-8 manifest")
    try object.close()
    archive = object.bytes
    input[0] = 9
    let reader = try await sdk.loadTDF(archive, readerOptions(raw))
    try check(try await reader.dataAttributes().count == 1, "attributes")
    try reader.close()
    var expected = original; expected.append(UInt8(i))
    try check(try await sdk.decryptBytes(archive, readerOptions(raw)) == expected, "round trip")
  }
  let platform = raw["PlatformURL"] as? String ?? ""
  var wrongOptions = optionsOf(raw, name); wrongOptions.clientSecret = "wrong"
  let wrong = try await expectFailure("wrong credentials") { () -> Data in
    let s = try await SDK(platform, wrongOptions)
    defer { try? s.close() }
    return try await s.decryptBytes(archive, readerOptions(raw))
  }
  try check(wrong["kind"] as? String == "auth" && wrong["httpStatus"] as? Int64 == 401, "wrong credentials rejected: \(wrong)")
  let started = DispatchSemaphore(value: 0)
  var heldOptions = optionsOf(raw, name, credentials: false)
  heldOptions.tokenProvider = { request, complete in
    started.signal()
    DispatchQueue.global().async {
      while !request.cancellation.isCanceled { Thread.sleep(forTimeInterval: 0.002) }
      complete(.failure(NSError(domain: "canceled-provider", code: 1)))
    }
    return nil
  }
  let holder = try await SDK(platform, heldOptions)
  let pending = Task { try await holder.decryptBytes(archive, readerOptions(raw)) }
  try check(started.wait(timeout: .now() + 20) == .success, "provider started")
  pending.cancel()
  let canceled = try await expectFailure("active cancel") { try await pending.value }
  try check(canceled["kind"] as? String == "canceled", "active provider cancel is typed: \(canceled)")
  try holder.close()
  let second = try await sdk.encryptBytes(Data("async\u{0}\u{ff}".utf8), tdfOptions(raw))
  try check(try await sdk.decryptBytes(second, readerOptions(raw)) == Data("async\u{0}\u{ff}".utf8), "second round trip")
  try sdk.close()
  try check((try await expectFailure("closed") { try await sdk.decryptBytes(archive, readerOptions(raw)) })["kind"] as? String == "closed", "closed handle")
}

/// One controlled case against the harness fixture (tests/interop/delivery/controlled_platform.py).
func controlled(_ run: URL, _ raw: [String: Any], _ name: String) async throws {
  let url = raw["PlatformURL"] as? String ?? ""
  let offline = TDFOptions(attributes: ["https://example.com/attr/a/value/b"], disableAutoconfigure: true, kasInformation: [KASInfo(url: "http://localhost:9999/malicious/kas", publicKey: raw["KASPublicKeyPEM"] as? String ?? "", kid: "k1", algorithm: "rsa:2048")])
  let credentials = Options(clientId: "opentdf", clientSecret: "secret")
  let error: [String: Any]
  switch name {
  case "unreachable": error = try await expectFailure(name) { try await SDK(url, Options(validateConnection: true)) }
  case "caller-deadline":
    let task = Task { try await SDK(url, credentials) }
    DispatchQueue.global().asyncAfter(deadline: .now() + 1) { task.cancel() }
    error = try await expectFailure(name) { try await task.value }
  case "source-deadline": error = try await expectFailure(name) { try await SDK(url, Options(clientId: "opentdf", clientSecret: "secret", timeoutMillis: 100)) }
  case "token-401":
    let sdk = try await SDK(url, credentials)
    let archive = try await sdk.encryptBytes(Data("offline payload".utf8), offline)
    error = try await expectFailure(name) { try await sdk.decryptBytes(archive, ReaderOptions(ignoreAllowlist: true)) }
    try sdk.close()
  case "offline":
    var rows: [String: Any] = [:]
    let sdk = try await SDK(url, credentials)
    let archive = try await sdk.encryptBytes(Data("offline payload".utf8), offline)
    rows["allowlist"] = try await expectFailure("allowlist") { try await sdk.decryptBytes(archive, ReaderOptions(kasAllowlist: [url])) }
    rows["invalid-tdf"] = try await expectFailure("invalid-tdf") { try await sdk.decryptBytes(Data("not a tdf".utf8)) }
    rows["unsupported-option"] = try await expectFailure("unsupported-option") { try await sdk.createTDF(Data("x".utf8), TDFOptions(targetMode: "4.2.2")) }
    for _ in 0..<100 {
      let reader = try await sdk.loadTDF(archive, ReaderOptions(ignoreAllowlist: true))
      try check(try await reader.manifest().encryptionInformation.keyAccess[0].kid == "k1", "manifest")
      try reader.close()
    }
    try sdk.close()
    rows["closed-handle"] = try await expectFailure("closed-handle") { try await sdk.decryptBytes(archive) }
    rows["handles"] = ["readers": 100, "closed": true]
    try check(try await SDK.isValidTdf(archive) && !(try SDK.version()).isEmpty && (try SDK.supportedFeatures()).contains("hexless"), "free functions")
    try writeJSON(run, "offline.json", rows)
    return
  default: error = try await expectFailure(name) { try await SDK(url, credentials) }
  }
  try writeJSON(run, name + ".error.json", error)
}

@main
struct Consumer {
  static func main() async throws {
    let args = CommandLine.arguments
    guard args.count == 5 else { fatalError("usage: consumer sdk run-dir mode name") }
    let run = URL(fileURLWithPath: args[2])
    let mode = args[3]
    let name = args[4]
    func path(_ suffix: String) -> URL { run.appendingPathComponent(name + suffix) }
    let raw = try JSONSerialization.jsonObject(with: Data(contentsOf: run.appendingPathComponent("config.json"))) as! [String: Any]
    do {
      if mode == "controlled" {
        try await controlled(run, raw, name)
        print("PASS generated native library", mode, name)
        return
      }
      let sdk = try await SDK(raw["PlatformURL"] as? String ?? "", optionsOf(raw, name))
      switch mode {
      case "encrypt":
        var options = tdfOptions(raw)
        if (name == "metadata" || name.hasSuffix("-metadata")) && name != "empty-metadata" && !name.hasSuffix("-empty-metadata") { options.metadata = "{\"source\":\"independent metadata\",\"count\":7}" }
        if name == "denied" || name.hasSuffix("-denied") { options.attributes = ["https://example.com/attr/attr1/value/value2"] }
        let object = try await sdk.createTDF(try Data(contentsOf: path(".input")), options)
        let size = try await object.size()
        let keyAccess = try await object.manifest().encryptionInformation.keyAccess
        try check(size == Int64(object.bytes.count) && keyAccess.count == 1, "object")
        try object.close()
        try object.bytes.write(to: path(".generated.tdf"))
      case "decrypt":
        let input = try Data(contentsOf: path(".tdf"))
        let reader = try await sdk.loadTDF(input, readerOptions(raw))
        let manifest = try await reader.manifest()
        let hasMetadata = !(manifest.encryptionInformation.keyAccess.first?.encryptedMetadata ?? "").isEmpty
        var metadata = Data()
        if hasMetadata { try await reader.initialize(); metadata = try await reader.unencryptedMetadata() }
        try reader.close()
        let payload = try await sdk.decryptBytes(input, readerOptions(raw))
        try payload.write(to: path(".out"))
        try metadata.write(to: path(".metadata"))
        try Data("\(manifest)".utf8).write(to: path(".manifest"))
        try Data((hasMetadata ? "true" : "false").utf8).write(to: path(".presence"))
      case "negative":
        try writeJSON(run, name + ".error.json", try await expectFailure("negative") { try await sdk.decryptBytes(try Data(contentsOf: path(".tdf")), readerOptions(raw)) })
      case "repeat":
        try await repeatRounds(sdk, raw, name)
        print("PASS generated native library", mode, name)
        return
      default: fatalError("unknown mode")
      }
      try sdk.close()
      print("PASS generated native library", mode, name)
    } catch let e as OpenTDFError {
      FileHandle.standardError.write(Data("consumer failure \(mode) \(name) \(report(e))\n".utf8))
      exit(1)
    }
  }
}
