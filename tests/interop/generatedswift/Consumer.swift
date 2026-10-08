import Foundation
import GoalchemyNative
import OpenTDFTDF3

#if canImport(FoundationNetworking)
  import FoundationNetworking
#endif

// Test caller's DPoP signer. Uses the native C ABI to exercise the typed token
// bridge independently of the shared source's OAuth/DPoP implementation.
private final class HostProof {
  let key: UnsafeMutableRawPointer
  let rsa: Bool
  let jwk: [String: String]
  let thumbprint: String
  static func b64(_ value: Data) -> String {
    value.base64EncodedString().replacingOccurrences(of: "+", with: "-")
      .replacingOccurrences(of: "/", with: "_").replacingOccurrences(of: "=", with: "")
  }
  init(_ pem: GoString, mismatch: Bool) throws {
    let imported =
      mismatch
      ? gcn_key_generate(2)
      : pem.bytes.withUnsafeBufferPointer { gcn_key_import($0.baseAddress, $0.count) }
    guard let imported = imported else { throw NSError(domain: "proof", code: 1) }
    key = imported
    rsa = gcn_key_kind(imported) == 1
    func component(_ index: Int32) throws -> String {
      var out = [UInt8](repeating: 0, count: 512)
      var length = out.count
      let status = out.withUnsafeMutableBufferPointer {
        gcn_key_component(imported, index, $0.baseAddress, &length)
      }
      guard status == 1 else { throw NSError(domain: "proof", code: 2) }
      return Self.b64(Data(out.prefix(length)))
    }
    jwk =
      rsa
      ? ["kty": "RSA", "n": try component(0), "e": try component(1)]
      : ["kty": "EC", "crv": "P-256", "x": try component(2), "y": try component(3)]
    let data = try JSONSerialization.data(
      withJSONObject: jwk, options: [.sortedKeys, .withoutEscapingSlashes])
    var digest = [UInt8](repeating: 0, count: 32)
    let status = data.withUnsafeBytes { source in
      digest.withUnsafeMutableBufferPointer {
        gcn_sha256(source.bindMemory(to: UInt8.self).baseAddress, data.count, $0.baseAddress)
      }
    }
    guard status == 1 else { throw NSError(domain: "proof", code: 3) }
    thumbprint = Self.b64(Data(digest))
  }
  deinit { gcn_key_release(key) }
  func sign(_ endpoint: String, _ nonce: String) throws -> String {
    let header: [String: Any] = ["typ": "dpop+jwt", "alg": rsa ? "RS256" : "ES256", "jwk": jwk]
    var claims: [String: Any] = [
      "htu": endpoint, "htm": "POST", "iat": Int64(Date().timeIntervalSince1970),
      "jti": UUID().uuidString,
    ]
    if !nonce.isEmpty { claims["nonce"] = nonce }
    let text =
      Self.b64(
        try JSONSerialization.data(
          withJSONObject: header, options: [.sortedKeys, .withoutEscapingSlashes])) + "."
      + Self.b64(
        try JSONSerialization.data(
          withJSONObject: claims, options: [.sortedKeys, .withoutEscapingSlashes]))
    let data = Data(text.utf8)
    var signature = [UInt8](repeating: 0, count: 512)
    var length = signature.count
    let status = data.withUnsafeBytes { source in
      signature.withUnsafeMutableBufferPointer {
        gcn_sign(
          rsa ? 1 : 2, key, source.bindMemory(to: UInt8.self).baseAddress, data.count,
          $0.baseAddress, &length)
      }
    }
    guard status == 1 else { throw NSError(domain: "proof", code: 4) }
    return text + "." + Self.b64(Data(signature.prefix(length)))
  }
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
    let raw =
      try JSONSerialization.jsonObject(
        with: Data(contentsOf: run.appendingPathComponent("config.json"))) as! [String: Any]
    func text(_ key: String) -> GoString { GoString(raw[key] as? String ?? "") }
    var config = Config()
    config.PlatformURL = text("PlatformURL")
    config.KASURL = text("KASURL")
    config.IssuerURL = text("IssuerURL")
    config.TokenURL = text("TokenURL")
    config.ClientID = text("ClientID")
    config.ClientSecret = text("ClientSecret")
    config.TokenProviderName = text("TokenProviderName")
    config.AllowHTTP = raw["AllowHTTP"] as? Bool ?? false
    config.TimeoutMillis = (raw["TimeoutMillis"] as? NSNumber)?.int64Value ?? 0
    config.KASPublicKeyPEM = text("KASPublicKeyPEM")
    config.KID = text("KID")
    config.KASAlgorithm = text("KASAlgorithm")
    config.SessionAlgorithm = text("SessionAlgorithm")
    config.AuthPrivateKeyPEM = text("AuthPrivateKeyPEM")
    config.AuthAlgorithm = text("AuthAlgorithm")
    config.DPoP = raw["DPoP"] as? Bool ?? false
    config.AllowedKAS = (raw["AllowedKAS"] as? [[String: String]])?.map {
      KASRoute(URL: GoString($0["URL"] ?? ""), APIBaseURL: GoString($0["APIBaseURL"] ?? ""))
    }
    func options(_ label: String) -> EncryptOptions {
      var result = EncryptOptions()
      let denied = label == "denied" || label.hasSuffix("-denied")
      result.Attributes = [
        GoString("https://example.com/attr/attr1/value/" + (denied ? "value2" : "value1"))
      ]
      result.SegmentSize = 16384
      result.HasSegmentSize = true
      if label == "metadata" || label.hasSuffix("-metadata") {
        result.Metadata = Array("{\"source\":\"independent metadata\",\"count\":7}".utf8)
        result.IncludeMetadata = true
      }
      if label == "empty-metadata" || label.hasSuffix("-empty-metadata") {
        result.Metadata = []
        result.IncludeMetadata = true
      }
      if label == "hs256" || label.hasSuffix("-hs256") { result.SegmentHashAlgorithm = "HS256" }
      return result
    }
    func writeError(_ error: TDFError) throws {
      let fields: [String: Any] = [
        "kind": error.kind, "code": error.code, "operation": error.operation,
        "httpStatus": error.httpStatus, "causeCategory": error.causeCategory,
        "serverCode": error.serverCode, "serverMessage": error.serverMessage,
      ]
      try JSONSerialization.data(withJSONObject: fields, options: [.sortedKeys]).write(
        to: path(".error.json"))
    }
    let provider: TokenProvider = { request, complete in
      if name == "provider-reject" {
        complete(.failure(NSError(domain: "fixture", code: 1)))
        return nil
      }
      if name == "invalid-token" || name == "expired-token" {
        complete(
          .success(
            AccessToken(
              value: "invalid", scheme: "Bearer",
              expiresAt: Int64(Date().timeIntervalSince1970) + (name == "expired-token" ? -1 : 300))
          ))
        return nil
      }
      // The public callback runs off the source scheduler. URLSession is
      // only the caller's token provider; SDK HTTP still uses native curl.
      let endpoint = config.IssuerURL.description + "/protocol/openid-connect/token"
      let proof: HostProof?
      do {
        proof =
          config.DPoP
          ? try HostProof(config.AuthPrivateKeyPEM, mismatch: name == "mismatched-provider") : nil
      } catch {
        complete(.failure(error))
        return nil
      }
      var activeTask: URLSessionDataTask?
      let taskLock = NSLock()
      func attempt(_ nonce: String, _ count: Int) {
        if request.cancellation.isCanceled {
          complete(.failure(NSError(domain: "canceled", code: 1)))
          return
        }
        var message = URLRequest(url: URL(string: endpoint)!)
        message.httpMethod = "POST"
        message.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")
        message.httpBody = Data(
          "grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret".utf8)
        do {
          if let proof = proof {
            message.setValue(try proof.sign(endpoint, nonce), forHTTPHeaderField: "DPoP")
          }
        } catch {
          complete(.failure(error))
          return
        }
        let task = URLSession.shared.dataTask(with: message) { data, response, error in
          do {
            if let error = error { throw error }
            if let response = response as? HTTPURLResponse,
              [400, 401].contains(response.statusCode), count < 3,
              let next = response.value(forHTTPHeaderField: "DPoP-Nonce"), !next.isEmpty,
              next != nonce
            {
              attempt(next, count + 1)
              return
            }
            guard (response as? HTTPURLResponse)?.statusCode == 200, let data = data,
              let value = try JSONSerialization.jsonObject(with: data) as? [String: Any],
              let token = value["access_token"] as? String,
              let seconds = value["expires_in"] as? NSNumber
            else {
              throw NSError(domain: "provider", code: 2)
            }
            complete(
              .success(
                AccessToken(
                  value: token, scheme: proof == nil ? "Bearer" : "DPoP",
                  expiresAt: Int64(Date().timeIntervalSince1970) + seconds.int64Value,
                  confirmationJKT: proof?.thumbprint ?? "")))
          } catch { complete(.failure(error)) }
        }
        taskLock.lock()
        activeTask = task
        taskLock.unlock()
        task.resume()
      }
      attempt("", 1)
      return {
        taskLock.lock()
        let task = activeTask
        taskLock.unlock()
        task?.cancel()
      }
    }
    let activeProvider: TokenProvider? = config.TokenProviderName.bytes.isEmpty ? nil : provider
    if mode == "encrypt" {
      try encrypt(config, Data(contentsOf: path(".input")), options(name), provider: activeProvider)
        .wait().write(to: path(".generated.tdf"))
    } else if mode == "decrypt" {
      let result = try await decrypt(
        config, Data(contentsOf: path(".tdf")), provider: activeProvider
      ).value()
      try result.payload.write(to: path(".out"))
      try result.metadata.write(to: path(".metadata"))
      try result.manifestJSON.write(to: path(".manifest"))
      try Data((result.hasMetadata ? "true" : "false").utf8).write(to: path(".presence"))
    } else if mode == "negative" {
      do {
        _ = try decrypt(config, Data(contentsOf: path(".tdf")), provider: activeProvider).wait()
        fatalError("negative plaintext")
      } catch let error as TDFError { try writeError(error) }
    } else if mode == "repeat" {
      var payload = Data([0, 255, 128, 1])
      var option = options("metadata")
      option.Metadata = [0, 255]
      option.MimeType = "application/日本語"
      let work = encrypt(config, payload, option)
      payload = Data("xxxx".utf8)
      option.Metadata = [120, 120]
      option.MimeType = "changed"
      let archive = try work.wait()
      let first = try decrypt(config, archive).wait()
      let second = try await decrypt(config, archive).value()
      precondition(first.payload == Data([0, 255, 128, 1]) && first.metadata == Data([0, 255]))
      precondition(first.payload == second.payload && first.metadata == second.metadata)
      precondition(String(decoding: first.manifestJSON, as: UTF8.self).contains("日本語"))
      let canceled = decrypt(config, archive)
      canceled.cancel()
      do {
        _ = try canceled.wait()
        fatalError("canceled plaintext")
      } catch let error as TDFError { precondition(error.kind == "canceled") }
      let entered = DispatchSemaphore(value: 0)
      let finished = DispatchSemaphore(value: 0)
      let held: TokenProvider = { request, complete in
        entered.signal()
        DispatchQueue.global().async {
          while !request.cancellation.isCanceled { Thread.sleep(forTimeInterval: 0.002) }
          complete(.failure(NSError(domain: "canceled-provider", code: 1)))
        }
        return nil
      }
      let pending = decrypt(config, archive, provider: held)
      DispatchQueue.global().async {
        defer { finished.signal() }
        do {
          _ = try pending.wait()
          fatalError("active canceled plaintext")
        } catch let error as TDFError { precondition(error.kind == "canceled") } catch {
          fatalError("untyped error")
        }
      }
      precondition(entered.wait(timeout: .now() + 15) == .success)
      pending.cancel()
      precondition(finished.wait(timeout: .now() + 15) == .success)
      let recovery = try decrypt(config, archive).wait()
      precondition(recovery.payload == first.payload)
    } else if mode == "controlled" {
      let fixed: TokenProvider = { _, complete in
        complete(
          .success(
            AccessToken(
              value: "negative-fixture-token", scheme: "Bearer",
              expiresAt: Int64(Date().timeIntervalSince1970) + 300)))
        return nil
      }
      let work = decrypt(
        config, try Data(contentsOf: run.appendingPathComponent("archive.tdf")), provider: fixed)
      if name == "active-cancel" {
        DispatchQueue.global().async {
          let end = Date().addingTimeInterval(15)
          while !FileManager.default.fileExists(atPath: run.appendingPathComponent("entered").path)
          {
            precondition(Date() < end)
            Thread.sleep(forTimeInterval: 0.002)
          }
          work.cancel()
        }
      }
      do {
        _ = try work.wait()
        fatalError("controlled plaintext")
      } catch let error as TDFError {
        let expected =
          [
            "http401": "unauthenticated", "http403": "denied", "redirect": "http_status",
            "content-type": "invalid_content_type",
            "malformed": "invalid_json", "active-cancel": "canceled", "bom": "invalid_destination",
          ][name] ?? "transport"
        if name == "active-cancel" {
          precondition(error.kind == "canceled" && error.causeCategory == "canceled")
        } else {
          precondition(error.code == expected, "unexpected error \(error.code)")
        }
        if ["http401", "http403"].contains(name) {
          precondition(error.serverCode == "fixture-rejected" && error.serverMessage == "診断 café")
        }
        try writeError(error)
      }
      do {
        _ = try decrypt(Config(), Data([0])).wait()
        fatalError("recovery")
      } catch let error as TDFError { precondition(error.kind == "source") }
    } else {
      fatalError("unknown mode")
    }
    print("PASS Swift importing consumer", mode, name)
  }
}
