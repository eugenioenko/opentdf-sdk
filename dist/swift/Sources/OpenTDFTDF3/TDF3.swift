import Foundation
import GoalchemyGenerated

// Configuration names preserve the shared Go fields, including binary GoString.
public typealias Config = GoalchemyGenerated.Config
public typealias KASRoute = GoalchemyGenerated.KASRoute
public typealias EncryptOptions = GoalchemyGenerated.EncryptOptions
public typealias GoString = GoalchemyGenerated.GoString
public typealias CallOptions = GoalchemyGenerated.CallOptions
public typealias CancellationToken = GoalchemyGenerated.CancellationToken
public typealias CallbackRequest = GoalchemyGenerated.CallbackRequest

public struct Decrypted {
  public let payload: Data
  public let metadata: Data
  public let hasMetadata: Bool
  public let manifestJSON: Data
}

public struct TDFError: Error, CustomStringConvertible {
  public let kind: String
  public let code: String
  public let operation: String
  public let httpStatus: Int64
  public let causeCategory: String
  public let serverCode: String
  public let serverMessage: String
  public let requiredObligations: [String]
  public var description: String { "tdf: \(code) (\(operation))" }

  init(_ failure: GoalchemyFailure) {
    func text(_ name: String) -> String {
      (failure.fields[name] as? GoString)?.description ?? ""
    }
    causeCategory = text("CauseCategory")
    kind = ["canceled", "deadline_exceeded"].contains(causeCategory) ? causeCategory : failure.kind
    code = text("Code").isEmpty ? kind : text("Code")
    operation = text("Operation")
    httpStatus = failure.fields["HTTPStatus"] as? Int64 ?? 0
    serverCode = text("ServerCode")
    serverMessage = text("ServerMessage")
    requiredObligations =
      (failure.fields["RequiredObligations"] as? [GoString])?.map(\.description) ?? []
  }
}

/// A lazy operation. Waiting starts the call; inputs are already owned.
public final class Operation<T> {
  private let synchronous: () throws -> T
  private let asynchronous: () async throws -> T
  private let cancelCall: () -> Void

  init<U>(_ native: GoalchemyGenerated.Operation<U>, _ convert: @escaping (U) -> T) {
    synchronous = { try convert(native.wait()) }
    asynchronous = { try convert(await native.value()) }
    cancelCall = { native.cancel() }
  }
  public func cancel() { cancelCall() }
  public func wait() throws -> T {
    do { return try synchronous() } catch let error as GoalchemyFailure { throw TDFError(error) }
  }
  public func value() async throws -> T {
    do { return try await asynchronous() } catch let error as GoalchemyFailure {
      throw TDFError(error)
    }
  }
}

public struct AccessToken {
  public let value: String
  public let scheme: String
  public let expiresAt: Int64
  public let confirmationJKT: String
  public init(value: String, scheme: String, expiresAt: Int64, confirmationJKT: String = "") {
    self.value = value
    self.scheme = scheme
    self.expiresAt = expiresAt
    self.confirmationJKT = confirmationJKT
  }
}

/// Complete once, including after cancellation; return an optional cleanup hook.
public typealias TokenProvider = (CallbackRequest, @escaping (Result<AccessToken, Error>) -> Void)
  -> (() -> Void)?

private func tokenOptions(_ config: Config, _ options: CallOptions, _ provider: TokenProvider?) -> (
  Config, CallOptions
) {
  guard let provider = provider else { return (config, options) }
  var config = config
  var options = options
  config.TokenProviderName = "access-token"
  options.callbacks["access-token"] = { request, completion in
    provider(request) { result in
      completion(
        result.flatMap { token in
          Result {
            guard token.expiresAt >= 0, token.value.utf8.count <= 65536,
              token.scheme.utf8.count <= 128, token.confirmationJKT.utf8.count <= 256
            else {
              throw GoalchemyFailure("host", "provider: invalid token")
            }
            let body = [
              "value": token.value, "scheme": token.scheme,
              "expiresAt": String(token.expiresAt), "confirmationJKT": token.confirmationJKT,
            ]
            return Array(try JSONSerialization.data(withJSONObject: body, options: [.sortedKeys]))
          }
        })
    }
  }
  return (config, options)
}

public func encrypt(
  _ config: Config, _ payload: Data, _ options: EncryptOptions = EncryptOptions(),
  callOptions: CallOptions = CallOptions(), provider: TokenProvider? = nil
) -> Operation<Data> {
  let (config, call) = tokenOptions(config, callOptions, provider)
  return Operation(GoalchemyGenerated.Encrypt(config, Array(payload), options, call)) {
    Data($0 ?? [])
  }
}

public func decrypt(
  _ config: Config, _ archive: Data,
  callOptions: CallOptions = CallOptions(), provider: TokenProvider? = nil
) -> Operation<Decrypted> {
  let (config, call) = tokenOptions(config, callOptions, provider)
  return Operation(GoalchemyGenerated.Decrypt(config, Array(archive), call)) {
    Decrypted(
      payload: Data($0.Payload ?? []), metadata: Data($0.Metadata ?? []),
      hasMetadata: $0.HasMetadata, manifestJSON: Data($0.ManifestJSON ?? []))
  }
}
