import Foundation
import OpenTDFTDF3

@main
struct Benchmark {
  static func main() throws {
    let args = CommandLine.arguments
    guard args.count == 7, args[2] == "e2e", let samples = Int(args[4]),
      let warmups = Int(args[5]), let bulkWarmups = Int(args[6]),
      samples > 0, warmups > 0, bulkWarmups >= 0
    else { fatalError("usage: benchmark directory e2e size samples warmups bulk-warmups") }
    let directory = URL(fileURLWithPath: args[1])
    func file(_ name: String) -> URL { directory.appendingPathComponent(name) }
    let raw =
      try JSONSerialization.jsonObject(with: Data(contentsOf: file("private.json")))
      as! [String: Any]
    let fields = raw["Config"] as! [String: Any]
    func text(_ name: String) -> GoString { GoString(fields[name] as? String ?? "") }
    var config = Config()
    config.PlatformURL = text("PlatformURL")
    config.KASURL = text("KASURL")
    config.AllowHTTP = fields["AllowHTTP"] as? Bool ?? false
    config.KASPublicKeyPEM = text("KASPublicKeyPEM")
    config.KID = text("KID")
    config.KASAlgorithm = text("KASAlgorithm")
    config.SessionAlgorithm = text("SessionAlgorithm")
    config.AuthAlgorithm = text("AuthAlgorithm")
    config.AllowedKAS = (fields["AllowedKAS"] as! [[String: String]]).map {
      KASRoute(URL: GoString($0["URL"]!), APIBaseURL: GoString($0["APIBaseURL"]!))
    }
    let token = AccessToken(
      value: raw["Token"] as! String, scheme: "Bearer",
      expiresAt: (raw["Expires"] as! NSNumber).int64Value)
    let provider: TokenProvider = { _, completion in
      completion(.success(token))
      return nil
    }
    let input = try Data(contentsOf: file(args[3] + ".input"))
    let bulkInput =
      bulkWarmups > 0 && args[3] != "50" ? try Data(contentsOf: file("50.input")) : input
    var options = EncryptOptions()
    options.Attributes = [GoString("https://example.com/attr/attr1/value/value1")]
    options.SegmentSize = 2 << 20
    options.HasSegmentSize = true
    options.SegmentHashAlgorithm = "GMAC"
    var measured: [Double] = []
    var warmupHistory: [Double] = []
    var bulkHistory: [Double] = []
    for index in (-bulkWarmups - warmups)..<samples {
      let payload = index < -warmups ? bulkInput : input
      let start = DispatchTime.now().uptimeNanoseconds
      let archive = try encrypt(config, payload, options, provider: provider).wait()
      let decrypted = try decrypt(config, archive, provider: provider).wait()
      let elapsed = Double(DispatchTime.now().uptimeNanoseconds - start) / 1e6
      guard decrypted.payload == payload else { fatalError("plaintext mismatch") }
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
