using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using System.Text;
using System.Text.Json;
using Generated = Goalchemy.Generated.GoProgram;
using Rt;
namespace OpenTDF;

/// TDF3 value facade. All protocol operations execute generated shared source.
public static class TDF3
{
    public sealed class KASRoute { public string URL = "", APIBaseURL = ""; }
    public sealed class Config
    {
        public string PlatformURL = "", KASURL = "", IssuerURL = "", TokenURL = "", ClientID = "", ClientSecret = "", TokenProviderName = "";
        public KASRoute[] AllowedKAS; public bool AllowHTTP, DPoP; public long TimeoutMillis;
        public string KASPublicKeyPEM = "", KID = "", KASAlgorithm = "", SessionAlgorithm = "", AuthPrivateKeyPEM = "", AuthAlgorithm = "";
    }
    public sealed class EncryptOptions
    { public string PolicyBase64 = "", SegmentHashAlgorithm = "", MimeType = ""; public string[] Attributes, Dissem; public long SegmentSize; public bool HasSegmentSize, IncludeMetadata; public byte[] Metadata; }
    public sealed record AccessToken(string Value, string Scheme, long ExpiresAt, string ConfirmationJKT = "");
    public sealed class Decrypted
    {
        readonly byte[] payload, metadata; public byte[] Payload => (byte[])payload.Clone(); public byte[] Metadata => (byte[])metadata.Clone(); public bool HasMetadata { get; }
        public string ManifestJSON { get; }
        internal Decrypted(byte[] payload, byte[] metadata, bool presence, string manifest) { this.payload = (byte[])payload.Clone(); this.metadata = (byte[])metadata.Clone(); HasMetadata = presence; ManifestJSON = manifest; }
    }
    public sealed class TDFError : Exception
    {
        public string Kind { get; }
        public string Code { get; }
        public string Operation { get; }
        public long HTTPStatus { get; }
        public string ServerCode { get; }
        public string ServerMessage { get; }
        public string CauseCategory { get; }
        readonly string[] obligations;
        public string[] RequiredObligations => (string[])obligations.Clone();
        internal TDFError(Library.Failure failure) : base("tdf: " + failure.Kind, failure.InnerException)
        { Kind = failure.Kind; var f = failure.Fields; Code = f.TryGetValue("Code", out var code) ? Text(code) : Kind; Operation = Field(f, "Operation"); HTTPStatus = f.TryGetValue("HTTPStatus", out var status) && status is long n ? n : 0; ServerCode = Field(f, "ServerCode"); ServerMessage = Field(f, "ServerMessage"); CauseCategory = Field(f, "CauseCategory"); if (f.TryGetValue("RequiredObligations", out var raw) && raw is object[] a) obligations = Array.ConvertAll(a, Text); else obligations = Array.Empty<string>(); }
    }
    public sealed class ProviderRequest
    {
        readonly Callback.Request request; internal ProviderRequest(Callback.Request request) { this.request = request; }
        public bool Canceled => request.Canceled; public void Retain() => request.Retain(); public void OnStop(Action stop) => request.OnStop(stop);
        public void Resolve(AccessToken token) { try { request.Resolve(TokenBytes(token)); } catch (ArgumentException) { request.Reject("provider: invalid token response"); } catch (Library.Failure) { request.Reject("provider: invalid token response"); } }
        public void Reject(string error) => request.Reject(error); public void Fault(Exception error) => request.Fault(error);
    }
    public delegate void TokenProvider(ProviderRequest request);
    public static TokenProvider FromTask(Func<ProviderRequest, Task<AccessToken>> provider) => request =>
    { var work = provider(request) ?? throw new InvalidOperationException("null provider Task"); request.Retain(); _ = Settle(request, work); };
    static async Task Settle(ProviderRequest request, Task<AccessToken> work) { try { request.Resolve(await work.ConfigureAwait(false)); } catch (Exception) { request.Reject("provider: rejected"); } }
    public sealed class CallOptions { public TokenProvider Provider; public CancellationToken Cancellation; }
    public sealed class Operation<T>
    {
        readonly Func<bool> cancel; public Task<T> Completion { get; }
        internal Operation(Func<bool> cancel, Task<T> completion) { this.cancel = cancel; Completion = completion; }
        public bool Cancel() => cancel();
    }
    static readonly UTF8Encoding UTF8 = new(false, true);
    static string Encode(string value) { try { return Encoding.Latin1.GetString(UTF8.GetBytes(value ?? "")); } catch (EncoderFallbackException) { throw Library.invalid(); } }
    static string Decode(string value) { foreach (char c in value) if (c > 255) throw new InvalidOperationException("invalid source text"); return UTF8.GetString(Encoding.Latin1.GetBytes(value)); }
    static string Text(object value) => value is string text ? Decode(text) : "";
    static string Field(Dictionary<string, object> fields, string name) => fields.TryGetValue(name, out var value) ? Text(value) : "";
    static string[] Strings(string[] input) { if (input == null) return null; Library.length(input.Length); return Array.ConvertAll(input, Encode); }
    static Generated.Config Configuration(Config input)
    {
        input ??= new Config(); var target = new Generated.Config();
        // Reflect plain native fields only; generated source objects are constructed later by the acquired owner.
        foreach (var field in typeof(Config).GetFields())
        {
            if (field.Name == "AllowedKAS") continue; object value = field.GetValue(input); if (value is string text) value = Encode(text); else if (field.FieldType == typeof(string)) value = "";
            typeof(Generated.Config).GetField(field.Name).SetValue(target, value);
        }
        var routes = input.AllowedKAS; if (routes != null) { Library.length(routes.Length); target.AllowedKAS = new Generated.KASRoute[routes.Length]; for (int i = 0; i < routes.Length; i++) { var r = routes[i] ?? throw Library.invalid(); target.AllowedKAS[i] = new Generated.KASRoute { URL = Encode(r.URL), APIBaseURL = Encode(r.APIBaseURL) }; } }
        return target;
    }
    static Generated.EncryptOptions Options(EncryptOptions input)
    { input ??= new EncryptOptions(); return new Generated.EncryptOptions { PolicyBase64 = Encode(input.PolicyBase64), SegmentHashAlgorithm = Encode(input.SegmentHashAlgorithm), MimeType = Encode(input.MimeType), Attributes = Strings(input.Attributes), Dissem = Strings(input.Dissem), SegmentSize = input.SegmentSize, HasSegmentSize = input.HasSegmentSize, IncludeMetadata = input.IncludeMetadata, Metadata = input.Metadata }; }
    static Library.Options Call(Generated.Config config, CallOptions input)
    { var options = new Library.Options { Cancellation = input?.Cancellation ?? default }; var provider = input?.Provider; if (provider != null) { config.TokenProviderName = "access-token"; options.Callbacks = new[] { new Callback.Registration("access-token", request => provider(new ProviderRequest(request))) }; } return options; }
    static Operation<R> Wrap<T, R>(Library.Operation<T> operation, Func<T, R> convert) => new(operation.Cancel, Convert(operation.Completion, convert));
    static async Task<R> Convert<T, R>(Task<T> work, Func<T, R> convert) { try { return convert(await work.ConfigureAwait(false)); } catch (Library.Failure failure) { throw new TDFError(failure); } }
    public static Operation<byte[]> Encrypt(Config config, byte[] input, EncryptOptions options = null, CallOptions callOptions = null)
    { try { var c = Configuration(config); var call = Call(c, callOptions); return Wrap(Generated.Encrypt(c, input, Options(options), call), value => (byte[])value.Clone()); } catch (Library.Failure e) { return Wrap(Library.failed<byte[]>(e), value => value); } }
    public static Operation<Decrypted> Decrypt(Config config, byte[] input, CallOptions callOptions = null)
    { try { var c = Configuration(config); return Wrap(Generated.Decrypt(c, input, Call(c, callOptions)), value => new Decrypted(value.Payload ?? Array.Empty<byte>(), value.Metadata ?? Array.Empty<byte>(), value.HasMetadata, UTF8.GetString(value.ManifestJSON))); } catch (Library.Failure e) { return Wrap(Library.failed<Decrypted>(e), value => value); } }
    public static Task<byte[]> EncryptAsync(Config config, byte[] input, EncryptOptions options = null, CancellationToken cancellation = default, TokenProvider provider = null) => Encrypt(config, input, options, new CallOptions { Cancellation = cancellation, Provider = provider }).Completion;
    public static Task<Decrypted> DecryptAsync(Config config, byte[] input, CancellationToken cancellation = default, TokenProvider provider = null) => Decrypt(config, input, new CallOptions { Cancellation = cancellation, Provider = provider }).Completion;
    static byte[] TokenBytes(AccessToken token)
    {
        if (token == null || token.Value == null || token.Scheme == null || token.ExpiresAt < 0 || token.Value.Length > 65536 || token.Scheme.Length > 128 || token.ConfirmationJKT?.Length > 256) throw new ArgumentException("invalid token");
        Encode(token.Value); Encode(token.Scheme); Encode(token.ConfirmationJKT);
        return JsonSerializer.SerializeToUtf8Bytes(new Dictionary<string, string> { { "value", token.Value }, { "scheme", token.Scheme }, { "expiresAt", token.ExpiresAt.ToString(System.Globalization.CultureInfo.InvariantCulture) }, { "confirmationJKT", token.ConfirmationJKT ?? "" } });
    }
}
