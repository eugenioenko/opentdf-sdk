using System;using System.IO;using System.Text;using System.Text.Json;using System.Collections.Generic;using System.Linq;using System.Threading;using System.Threading.Tasks;using System.Security.Cryptography;using System.Net.Http;using OpenTDF;
namespace Consumer;

/// Independent assembly consumer with native .NET OAuth/DPoP provider.
static class Entry
{
 static string run;static JsonElement raw;static TDF3.Config config;static readonly HttpClient HTTP=new(new SocketsHttpHandler{AllowAutoRedirect=false}){Timeout=TimeSpan.FromSeconds(15)};
 static string Text(JsonElement value,string name)=>value.TryGetProperty(name,out var result)?result.ValueKind==JsonValueKind.String?result.GetString():result.ToString():"";
 static bool Bool(JsonElement value,string name)=>value.TryGetProperty(name,out var result)&&result.ValueKind==JsonValueKind.True;
 static TDF3.Config Config(JsonElement value)
 {var result=new TDF3.Config();foreach(var field in typeof(TDF3.Config).GetFields()){if(field.FieldType==typeof(string))field.SetValue(result,Text(value,field.Name));else if(field.FieldType==typeof(bool))field.SetValue(result,Bool(value,field.Name));else if(field.FieldType==typeof(long))field.SetValue(result,Text(value,field.Name)==""?0:long.Parse(Text(value,field.Name)));}if(value.TryGetProperty("AllowedKAS",out var routes))result.AllowedKAS=routes.EnumerateArray().Select(v=>new TDF3.KASRoute{URL=Text(v,"URL"),APIBaseURL=Text(v,"APIBaseURL")}).ToArray();return result;}
 static void Check(bool ok,string name){if(!ok)throw new Exception("ASSERT: "+name);}
 static byte[] Read(string name)=>File.ReadAllBytes(Path.Combine(run,name));static void Write(string name,byte[] value)=>File.WriteAllBytes(Path.Combine(run,name),value);static void Write(string name,string value)=>File.WriteAllText(Path.Combine(run,name),value,new UTF8Encoding(false));
 static T Await<T>(TDF3.Operation<T> operation)=>operation.Completion.WaitAsync(TimeSpan.FromSeconds(90)).GetAwaiter().GetResult();
 static string B64(byte[] value)=>Convert.ToBase64String(value).TrimEnd('=').Replace('+','-').Replace('/','_');
 static async Task<TDF3.AccessToken> Token(string name,TDF3.ProviderRequest request)
 {
  long now=DateTimeOffset.UtcNow.ToUnixTimeSeconds();if(name=="invalid-token")return new("invalid-token","Bearer",now+300);if(name=="expired-token")return new("expired-token","Bearer",now-1);if(name=="provider-reject")throw new ArgumentException("provider rejected");
  const string endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";string nonce="",thumb="";RSA rsa=null;ECDsa ec=null;SortedDictionary<string,string> jwk=null;
  using var cancel=new CancellationTokenSource();request.OnStop(()=>cancel.Cancel());
  try
  {
   if(config.DPoP)
   {
    if(name=="mismatched-provider")ec=ECDsa.Create(ECCurve.NamedCurves.nistP256);
    else{try{rsa=RSA.Create();rsa.ImportFromPem(config.AuthPrivateKeyPEM);}catch(CryptographicException){rsa?.Dispose();rsa=null;ec=ECDsa.Create();ec.ImportFromPem(config.AuthPrivateKeyPEM);}}
    jwk=new(StringComparer.Ordinal);if(rsa!=null){var p=rsa.ExportParameters(false);jwk["e"]=B64(p.Exponent);jwk["kty"]="RSA";jwk["n"]=B64(p.Modulus);}else{var p=ec.ExportParameters(false);jwk["crv"]="P-256";jwk["kty"]="EC";jwk["x"]=B64(p.Q.X);jwk["y"]=B64(p.Q.Y);}thumb=B64(SHA256.HashData(JsonSerializer.SerializeToUtf8Bytes(jwk)));
   }
   for(int attempt=0;attempt<3;attempt++)
   {
    using var message=new HttpRequestMessage(HttpMethod.Post,endpoint){Content=new FormUrlEncodedContent(new Dictionary<string,string>{{"grant_type","client_credentials"},{"client_id","opentdf-sdk"},{"client_secret","secret"}})};
    if(config.DPoP)
    {var header=new Dictionary<string,object>{{"typ","dpop+jwt"},{"alg",rsa!=null?"RS256":"ES256"},{"jwk",jwk}};var claims=new Dictionary<string,object>{{"htu",endpoint},{"htm","POST"},{"iat",DateTimeOffset.UtcNow.ToUnixTimeSeconds()},{"jti",B64(RandomNumberGenerator.GetBytes(16))}};if(nonce!="")claims["nonce"]=nonce;string data=B64(JsonSerializer.SerializeToUtf8Bytes(header))+"."+B64(JsonSerializer.SerializeToUtf8Bytes(claims));var bytes=Encoding.ASCII.GetBytes(data);byte[] sig=rsa!=null?rsa.SignData(bytes,HashAlgorithmName.SHA256,RSASignaturePadding.Pkcs1):ec.SignData(bytes,HashAlgorithmName.SHA256,DSASignatureFormat.IeeeP1363FixedFieldConcatenation);message.Headers.TryAddWithoutValidation("DPoP",data+"."+B64(sig));}
    using var response=await HTTP.SendAsync(message,cancel.Token);byte[] body=await response.Content.ReadAsByteArrayAsync(cancel.Token);if(body.Length>128<<10)throw new ArgumentException("provider body limit");string next=response.Headers.TryGetValues("DPoP-Nonce",out var nonces)?nonces.First():"";
    if(((int)response.StatusCode==400||(int)response.StatusCode==401)&&next!=""&&next!=nonce){nonce=next;continue;}if((int)response.StatusCode!=200)throw new ArgumentException("provider status");var value=JsonDocument.Parse(body).RootElement;return new(Text(value,"access_token"),config.DPoP?"DPoP":"Bearer",DateTimeOffset.UtcNow.ToUnixTimeSeconds()+long.Parse(Text(value,"expires_in")),thumb);
   }
   throw new ArgumentException("provider nonce retries");
  }
  finally{rsa?.Dispose();ec?.Dispose();}
 }
 static TDF3.CallOptions Call(string name)=>new(){Provider=config.TokenProviderName==""?null:TDF3.FromTask(request=>Token(name,request))};
 static TDF3.EncryptOptions Opts(string name)
 {var result=new TDF3.EncryptOptions{Attributes=new[]{"https://example.com/attr/attr1/value/"+(name=="denied"||name.EndsWith("-denied")?"value2":"value1")},SegmentSize=16384,HasSegmentSize=true};if(name=="metadata"||name.EndsWith("-metadata")){result.Metadata=Encoding.UTF8.GetBytes("{\"source\":\"independent metadata\",\"count\":7}");result.IncludeMetadata=true;}if(name=="empty-metadata"||name.EndsWith("-empty-metadata")){result.Metadata=Array.Empty<byte>();result.IncludeMetadata=true;}if(name=="hs256"||name.EndsWith("-hs256"))result.SegmentHashAlgorithm="HS256";return result;}
 static void Error(string name,TDF3.TDFError e)=>Write(name+".error.json",JsonSerializer.Serialize(new{kind=e.Kind,code=e.Code,operation=e.Operation,httpStatus=e.HTTPStatus,causeCategory=e.CauseCategory}));
 static void Repeat()
 {
  byte[] payload={0,255,128,1},metadata={0,255};var options=Opts("metadata");options.Metadata=metadata;options.MimeType="application/日本語";var archive=Await(TDF3.Encrypt(config,payload,options,Call("ownership")));var a=Await(TDF3.Decrypt(config,archive,Call("ownership")));var b=Await(TDF3.Decrypt(config,archive,Call("ownership")));Check(a.Payload.SequenceEqual(payload)&&a.Metadata.SequenceEqual(metadata),"durable bytes");Check(a.ManifestJSON.Contains("日本語"),"UTF8 manifest");var own=b.Payload;own[0]=44;metadata[0]=44;Check(a.Payload[0]==0&&a.Metadata[0]==0,"owned output");
  using var started=new ManualResetEventSlim();var held=new TDF3.CallOptions{Provider=request=>{request.Retain();request.OnStop(()=>request.Reject("provider: stopped"));started.Set();}};var fresh=Config(raw);fresh.ClientID="";fresh.ClientSecret="";var active=TDF3.Decrypt(fresh,archive,held);Check(started.Wait(20000),"provider started");var queued=TDF3.Decrypt(config,archive,Call("ownership"));Check(queued.Cancel(),"queued cancel");try{Await(queued);throw new Exception("queued success");}catch(TDF3.TDFError e){Check(e.Kind=="canceled","queued category");}Check(active.Cancel(),"active cancel");try{Await(active);throw new Exception("active success");}catch(TDF3.TDFError e){Check(e.Kind=="canceled","active category");}Check(Await(TDF3.Decrypt(config,archive,Call("ownership"))).Payload.SequenceEqual(payload),"nextcall recovery");
 }
 static void Controlled(string name)
 {
  var options=new TDF3.CallOptions{Provider=request=>request.Resolve(new("negative-fixture-token","Bearer",DateTimeOffset.UtcNow.ToUnixTimeSeconds()+300))};var operation=TDF3.Decrypt(config,Read("archive.tdf"),options);Task cancel=null;
  if(name=="active-cancel")cancel=Task.Run(async()=>{var end=DateTime.UtcNow.AddSeconds(15);while(!File.Exists(Path.Combine(run,"entered"))){if(DateTime.UtcNow>end)throw new Exception("HTTP not acquired");await Task.Delay(2);}operation.Cancel();});
  TDF3.TDFError error;try{Await(operation);throw new Exception("controlled plaintext");}catch(TDF3.TDFError e){error=e;}
  string expected=name switch{"http401"=>"unauthenticated","http403"=>"denied","redirect"=>"http_status","content-type"=>"invalid_content_type","malformed"=>"invalid_json","active-cancel"=>"canceled","bom"=>"invalid_destination",_=>"transport"};Check(error.Code==expected,"controlled category "+name+" "+error.Code);if(name=="http401"||name=="http403")Check(error.ServerCode=="fixture-rejected"&&error.ServerMessage=="診断 café","UTF8 server error");cancel?.GetAwaiter().GetResult();Error(name,error);try{Await(TDF3.Decrypt(new(),new byte[]{0}));throw new Exception("invalid config success");}catch(TDF3.TDFError e){Check(e.Kind=="source","recovery");}
 }
 public static void Main(string[] args)
 {
  if(args.Length!=4)throw new ArgumentException("consumer sdk run mode case");run=args[1];string mode=args[2],name=args[3];raw=JsonDocument.Parse(File.ReadAllText(Path.Combine(run,"config.json"))).RootElement;config=Config(raw);
  try{switch(mode){case "encrypt":Write(name+".generated.tdf",Await(TDF3.Encrypt(config,Read(name+".input"),Opts(name),Call(name))));break;case "decrypt":var value=Await(TDF3.Decrypt(config,Read(name+".tdf"),Call(name)));Write(name+".out",value.Payload);Write(name+".metadata",value.Metadata);Write(name+".manifest",value.ManifestJSON);Write(name+".presence",value.HasMetadata?"true":"false");break;case "negative":try{Await(TDF3.Decrypt(config,Read(name+".tdf"),Call(name)));throw new Exception("negative plaintext");}catch(TDF3.TDFError e){Error(name,e);}break;case "repeat":Repeat();break;case "controlled":Controlled(name);break;default:throw new ArgumentException("unknown mode");}}
  catch(TDF3.TDFError e){throw new Exception("consumer failure "+mode+" "+name+" kind="+e.Kind+" code="+e.Code+" operation="+e.Operation+" cause="+e.CauseCategory,e);}
 }
}
