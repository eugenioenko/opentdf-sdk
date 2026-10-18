import Foundation
import OpenTDFTDF3

// End-to-end installed Swift handle-API consumer: one SDK per process, as the reference consumer.
@main
struct Benchmark {
  static func main() async throws {
    let args = CommandLine.arguments
    guard args.count == 7, args[2] == "e2e" || args[2] == "validate"
    else { fatalError("usage: benchmark directory e2e size samples warmups bulk-warmups; or directory validate size archive 0 0") }
    let validating = args[2] == "validate"
    let samples = validating ? 0 : Int(args[4]) ?? 0
    let warmups = Int(args[5]) ?? -1
    let bulkWarmups = Int(args[6]) ?? -1
    guard validating || (samples > 0 && warmups > 0 && bulkWarmups >= 0)
    else { fatalError("invalid benchmark policy") }
    let directory = URL(fileURLWithPath: args[1])
    func file(_ name: String) -> URL { directory.appendingPathComponent(name) }
    let raw =
      try JSONSerialization.jsonObject(with: Data(contentsOf: file("private.json")))
      as! [String: Any]
    let fields = raw["Config"] as! [String: Any]
    func text(_ name: String) -> String { fields[name] as? String ?? "" }
    let kasUrl = text("KASURL")
    let token = AccessToken(
      value: raw["Token"] as! String, scheme: "Bearer",
      expiresAt: (raw["Expires"] as! NSNumber).int64Value)
    let provider: TokenProvider = { _, completion in
      completion(.success(token))
      return nil
    }
    let sdk = try await SDK(
      text("PlatformURL"),
      Options(tokenEndpoint: text("TokenURL"), tokenProvider: provider, insecurePlaintext: true))
    defer { try? sdk.close() }
    let options = TDFOptions(
      attributes: ["https://example.com/attr/attr1/value/value1"], disableAutoconfigure: true,
      kasInformation: [
        KASInfo(url: kasUrl, publicKey: text("KASPublicKeyPEM"), kid: text("KID"), algorithm: "rsa:2048")
      ],
      wrappingKeyAlgorithm: "rsa:2048", segmentSize: 2 << 20)
    // LoadTDF reuses the SDK's default RSA2048 session. Do not generate a replacement.
    let readerOptions = ReaderOptions(kasAllowlist: [kasUrl])
    let input = try Data(contentsOf: file(args[3] + ".input"))
    if validating {
      let archive = try Data(contentsOf: URL(fileURLWithPath: args[4]))
      let decrypted = try await sdk.decryptBytes(archive, readerOptions)
      guard decrypted == input else { fatalError("independent stock Go plaintext mismatch") }
      FileHandle.standardOutput.write(Data("{\"correct\":true}\n".utf8))
      return
    }
    let bulkInput =
      bulkWarmups > 0 && args[3] != "50" ? try Data(contentsOf: file("50.input")) : input
    var measured: [Double] = []
    var warmupHistory: [Double] = []
    var bulkHistory: [Double] = []
    for index in (-bulkWarmups - warmups)..<samples {
      let payload = index < -warmups ? bulkInput : input
      let start = DispatchTime.now().uptimeNanoseconds
      let archive = try await sdk.encryptBytes(payload, options)
      let decrypted = try await sdk.decryptBytes(archive, readerOptions)
      let elapsed = Double(DispatchTime.now().uptimeNanoseconds - start) / 1e6
      guard decrypted == payload else { fatalError("plaintext mismatch") }
      if index == -1 || index >= 0 {
        try archive.write(to: file("swift-\(args[3])-\(index).tdf"))
      }
      if index >= 0 {
        measured.append(elapsed)
      } else if index < -warmups {
        bulkHistory.append(elapsed)
      } else {
        warmupHistory.append(elapsed)
      }
    }
    let result: [String: Any] = [
      "samples_ms": measured, "warmup_ms": warmupHistory,
      "bulk_warmup_ms": bulkHistory, "warmup_count": warmups,
      "bulk_warmup_count": bulkWarmups, "correct": true,
      "kas_calls_expected": samples + warmups + bulkWarmups,
    ]
    let output = try JSONSerialization.data(withJSONObject: result, options: [.sortedKeys])
    FileHandle.standardOutput.write(output)
    FileHandle.standardOutput.write(Data([10]))
  }
}
