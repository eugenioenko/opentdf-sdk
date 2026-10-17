using System;using System.IO;using System.Text;using System.Text.Json;using System.Collections.Generic;using System.Linq;using System.Threading;using System.Threading.Tasks;using System.Security.Cryptography;using System.Net.Http;using OpenTDF;
using Attributes=OpenTDF.Protocol.Policy.Attributes;using Namespaces=OpenTDF.Protocol.Policy.Namespaces;using Policy=OpenTDF.Protocol.Policy;
namespace Consumer;

/// Independent assembly consumer of the handle API: one SDK per run, CreateTdfAsync/DecryptBytesAsync, LoadTdfAsync readers, 100 round trips, the platform service clients, and the controlled checks of tests/interop/delivery/controlled_platform.py. Host OAuth/DPoP is native .NET.
static class Entry
{
 static string run,name;static JsonElement raw;static readonly HttpClient HTTP=new(new SocketsHttpHandler{AllowAutoRedirect=false}){Timeout=TimeSpan.FromSeconds(15)};
 const string Attribute="https://example.com/attr/attr1/value/value1";
 static string Text(JsonElement value,string key)=>value.TryGetProperty(key,out var result)?result.ValueKind==JsonValueKind.String?result.GetString():result.ToString():"";
 static bool Bool(JsonElement value,string key)=>value.TryGetProperty(key,out var result)&&result.ValueKind==JsonValueKind.True;
 static void Check(bool ok,string label){if(!ok)throw new Exception("ASSERT: "+label);}
 static byte[] Read(string file)=>File.ReadAllBytes(Path.Combine(run,file));static void Write(string file,byte[] value)=>File.WriteAllBytes(Path.Combine(run,file),value);static void Write(string file,string value)=>File.WriteAllText(Path.Combine(run,file),value,new UTF8Encoding(false));
 static string B64(byte[] value)=>Convert.ToBase64String(value).TrimEnd('=').Replace('+','-').Replace('/','_');
 // The host token source: native OAuth client credentials with a DPoP proof when the profile asks for one.
 static async Task<TDF3.AccessToken> Token(JsonElement cfg,TDF3.ProviderRequest request)
 {
  long now=DateTimeOffset.UtcNow.ToUnixTimeSeconds();if(name=="invalid-token")return new("invalid-token","Bearer",now+300);if(name=="expired-token")return new("expired-token","Bearer",now-1);if(name=="provider-reject")throw new ArgumentException("provider rejected");
  const string endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";string nonce="",thumb="";RSA rsa=null;ECDsa ec=null;SortedDictionary<string,string> jwk=null;bool dpop=Bool(cfg,"DPoP");
  using var cancel=new CancellationTokenSource();request.OnStop(()=>cancel.Cancel());
  try
  {
   if(dpop)
   {
    if(name=="mismatched-provider")ec=ECDsa.Create(ECCurve.NamedCurves.nistP256);
    else{try{rsa=RSA.Create();rsa.ImportFromPem(Text(cfg,"AuthPrivateKeyPEM"));}catch(CryptographicException){rsa?.Dispose();rsa=null;ec=ECDsa.Create();ec.ImportFromPem(Text(cfg,"AuthPrivateKeyPEM"));}}
    jwk=new(StringComparer.Ordinal);if(rsa!=null){var p=rsa.ExportParameters(false);jwk["e"]=B64(p.Exponent);jwk["kty"]="RSA";jwk["n"]=B64(p.Modulus);}else{var p=ec.ExportParameters(false);jwk["crv"]="P-256";jwk["kty"]="EC";jwk["x"]=B64(p.Q.X);jwk["y"]=B64(p.Q.Y);}thumb=B64(SHA256.HashData(JsonSerializer.SerializeToUtf8Bytes(jwk)));
   }
   for(int attempt=0;attempt<3;attempt++)
   {
    using var message=new HttpRequestMessage(HttpMethod.Post,endpoint){Content=new FormUrlEncodedContent(new Dictionary<string,string>{{"grant_type","client_credentials"},{"client_id","opentdf-sdk"},{"client_secret","secret"}})};
    if(dpop)
    {var header=new Dictionary<string,object>{{"typ","dpop+jwt"},{"alg",rsa!=null?"RS256":"ES256"},{"jwk",jwk}};var claims=new Dictionary<string,object>{{"htu",endpoint},{"htm","POST"},{"iat",DateTimeOffset.UtcNow.ToUnixTimeSeconds()},{"jti",B64(RandomNumberGenerator.GetBytes(16))}};if(nonce!="")claims["nonce"]=nonce;string data=B64(JsonSerializer.SerializeToUtf8Bytes(header))+"."+B64(JsonSerializer.SerializeToUtf8Bytes(claims));var bytes=Encoding.ASCII.GetBytes(data);byte[] sig=rsa!=null?rsa.SignData(bytes,HashAlgorithmName.SHA256,RSASignaturePadding.Pkcs1):ec.SignData(bytes,HashAlgorithmName.SHA256,DSASignatureFormat.IeeeP1363FixedFieldConcatenation);message.Headers.TryAddWithoutValidation("DPoP",data+"."+B64(sig));}
    using var response=await HTTP.SendAsync(message,cancel.Token);byte[] body=await response.Content.ReadAsByteArrayAsync(cancel.Token);if(body.Length>128<<10)throw new ArgumentException("provider body limit");string next=response.Headers.TryGetValues("DPoP-Nonce",out var nonces)?nonces.First():"";
    if(((int)response.StatusCode==400||(int)response.StatusCode==401)&&next!=""&&next!=nonce){nonce=next;continue;}if((int)response.StatusCode!=200)throw new ArgumentException("provider status");var value=JsonDocument.Parse(body).RootElement;return new(Text(value,"access_token"),dpop?"DPoP":"Bearer",DateTimeOffset.UtcNow.ToUnixTimeSeconds()+long.Parse(Text(value,"expires_in")),thumb);
   }
   throw new ArgumentException("provider nonce retries");
  }
  finally{rsa?.Dispose();ec?.Dispose();}
 }
 // config.json carries the façade Config fields the delivery scripts write; they map onto the handle API's options as the Go consumer maps them.
 static Options OptionsOf(JsonElement cfg,bool credentials=true)=>new()
 {
  InsecurePlaintext=Bool(cfg,"AllowHTTP"),TokenEndpoint=Text(cfg,"TokenURL")==""?null:Text(cfg,"TokenURL"),
  ClientId=credentials&&Text(cfg,"ClientID")!=""?Text(cfg,"ClientID"):null,ClientSecret=credentials&&Text(cfg,"ClientID")!=""?Text(cfg,"ClientSecret"):null,
  TokenProvider=Text(cfg,"TokenProviderName")==""?null:TDF3.FromTask(request=>Token(cfg,request)),
  Timeout=Text(cfg,"TimeoutMillis")==""?null:TimeSpan.FromMilliseconds(long.Parse(Text(cfg,"TimeoutMillis"))),
  DpopAlgorithm=Bool(cfg,"DPoP")?(Text(cfg,"AuthAlgorithm")==""?"ES256":Text(cfg,"AuthAlgorithm")):null,DpopKeyPem=Text(cfg,"AuthPrivateKeyPEM")==""?null:Text(cfg,"AuthPrivateKeyPEM"),
 };
 static KasInfo KasInformation(JsonElement cfg)=>new(Text(cfg,"KASURL")==""?Text(cfg,"PlatformURL").TrimEnd('/')+"/kas":Text(cfg,"KASURL")){PublicKey=Text(cfg,"KASPublicKeyPEM"),Kid=Text(cfg,"KID"),Algorithm=Text(cfg,"KASAlgorithm")};
 static TdfOptions TdfOptionsOf(JsonElement cfg)=>new(){DisableAutoconfigure=true,KasInformation=new[]{KasInformation(cfg)},Attributes=new[]{Attribute},SegmentSize=16384,WrappingKeyAlgorithm=Text(cfg,"KASAlgorithm")==""?null:Text(cfg,"KASAlgorithm")};
 static ReaderOptions ReaderOptionsOf(JsonElement cfg)
 {
  var allow=new List<string>{Text(cfg,"PlatformURL")};if(Text(cfg,"KASURL")!="")allow.Add(Text(cfg,"KASURL"));
  if(cfg.TryGetProperty("AllowedKAS",out var routes))foreach(var r in routes.EnumerateArray())allow.Add(Text(r,"URL"));
  return new(){KasAllowlist=allow,SessionKeyType=Text(cfg,"SessionAlgorithm")==""?null:Text(cfg,"SessionAlgorithm")};
 }
 static Dictionary<string,object> Report(Exception e)=>e is OpenTdfException o?new(){["kind"]=o.Kind,["op"]=o.Op,["httpStatus"]=o.HttpStatus,["connectCode"]=o.ConnectCode,["message"]=o.Message}:new(){["untyped"]=e.GetType().Name+": "+e.Message};
 static async Task<Dictionary<string,object>> ExpectFailure(string label,Func<Task> call)
 {
  try{await call();}catch(OpenTdfException e){return Report(e);}catch(Exception e){throw new Exception(label+" untyped: "+e);}
  throw new Exception(label+" returned successful output");
 }
 static IReadOnlyList<object> KeyAccess(IReadOnlyDictionary<string,object> manifest)=>(IReadOnlyList<object>)((IReadOnlyDictionary<string,object>)manifest["encryptionInformation"])["keyAccess"];

 /// 100 encrypt and decrypt calls on one SDK, independent inputs and results, every handle closed; then the SDK is closed and later calls fail.
 /// xtest's test_self.py through the service clients: list namespaces, create a namespace and an attribute, read the attribute back by id, through the namespace's list and through the raw JSON call, and check a missing attribute's error.
 static async Task Services(Sdk sdk,JsonElement cfg)
 {
  string ns="gccs"+Convert.ToHexString(RandomNumberGenerator.GetBytes(6)).ToLowerInvariant()+".com";
  var before=await sdk.Namespaces.ListNamespacesAsync(new Namespaces.ListNamespacesRequest());
  var created=(await sdk.Namespaces.CreateNamespaceAsync(new Namespaces.CreateNamespaceRequest{Name=ns})).Namespace;
  Check(created.Id!=""&&created.Name==ns,"created namespace");
  var after=await sdk.Namespaces.ListNamespacesAsync(new Namespaces.ListNamespacesRequest());
  Check(after.Namespaces.Any(n=>n.Id==created.Id)&&after.Namespaces.Count==before.Namespaces.Count+1,"namespace listed");
  var request=new Attributes.CreateAttributeRequest{NamespaceId=created.Id,Name="free",Rule=Policy.AttributeRuleTypeEnum.AnyOf};request.Values.Add(new[]{"1","2","3"});
  var attribute=(await sdk.Attributes.GetAttributeAsync(new Attributes.GetAttributeRequest{AttributeId=(await sdk.Attributes.CreateAttributeAsync(request)).Attribute.Id})).Attribute;
  var values=attribute.Values.Select(v=>v.Value_).ToArray();
  Check(attribute.Fqn=="https://"+ns+"/attr/free"&&attribute.Rule==Policy.AttributeRuleTypeEnum.AnyOf&&values.SequenceEqual(new[]{"1","2","3"})&&attribute.Namespace.Name==ns,"attribute read back");
  var listed=await sdk.Attributes.ListAttributesAsync(new Attributes.ListAttributesRequest{Namespace=ns});
  Check(listed.Attributes.Count==1&&listed.Attributes[0].Id==attribute.Id,"attribute listed");
  var raw=JsonDocument.Parse(await sdk.CallAsync("policy.attributes.AttributesService","GetAttribute",JsonSerializer.Serialize(new Dictionary<string,string>{["attributeId"]=attribute.Id}))).RootElement;
  Check(raw.GetProperty("attribute").GetProperty("fqn").GetString()==attribute.Fqn,"raw JSON call");
  var missing=await ExpectFailure("missing attribute",()=>sdk.Attributes.GetAttributeAsync(new Attributes.GetAttributeRequest{AttributeId="00000000-0000-0000-0000-000000000000"}));
  Check((string)missing["connectCode"]=="not_found"&&(long)missing["httpStatus"]==404,"missing attribute error: "+JsonSerializer.Serialize(missing));
  Write("services.json",JsonSerializer.Serialize(new Dictionary<string,object>{["namespace"]=ns,["namespaces_before"]=before.Namespaces.Count,["namespaces_after"]=after.Namespaces.Count,["attribute_fqn"]=attribute.Fqn,["values"]=values,["not_found"]=missing,["helpers"]=await Helpers(sdk,cfg,ns,attribute.Fqn),["autoconfigure"]=await Autoconfigure(sdk,cfg,attribute)}));
 }
 // The reference's attribute helpers on the attribute services created, Sdk.Auth, and an Sdk whose platform endpoint is unreachable but whose core connection is the platform.
 /// Encrypts under a value of the attribute created, looked up (Attributes) and offline (AttributeValues): the attribute has no grants, so the plan is the base key or the default KAS.
 static async Task<Dictionary<string,object>> Autoconfigure(Sdk sdk,JsonElement cfg,Policy.Attribute attribute)
 {
  string value=attribute.Fqn+"/value/1";
  var offline=new Policy.Value{Fqn=value,Value_="1",Attribute=new Policy.Attribute{Fqn=attribute.Fqn,Rule=attribute.Rule,Namespace=attribute.Namespace}};
  var report=new Dictionary<string,object>{["key"]=(await Sdk.NewAttributeValueFqnAsync(value)).Key};
  foreach(var (label,options) in new[]{("looked_up",new TdfOptions{KasInformation=new[]{KasInformation(cfg)},Attributes=new[]{value}}),("offline",new TdfOptions{KasInformation=new[]{KasInformation(cfg)},AttributeValues=new[]{offline}})})
  {
   await using var obj=await sdk.CreateTdfAsync(Encoding.UTF8.GetBytes(label),options);
   var kaos=KeyAccess(await obj.ManifestAsync());var kao=(IReadOnlyDictionary<string,object>)kaos[0];
   report[label]=new Dictionary<string,object>{["kaos"]=kaos.Count,["kas"]=kao["url"],["sid"]=kao.TryGetValue("sid",out var sid)?sid:""};
  }
  return report;
 }
 static async Task<Dictionary<string,object>> Helpers(Sdk sdk,JsonElement cfg,string ns,string fqn)
 {
  var h=new Dictionary<string,object>{["attribute_exists"]=await sdk.AttributeExistsAsync(fqn),["attribute_missing"]=await sdk.AttributeExistsAsync(fqn+"-missing"),
   ["value_exists"]=await sdk.AttributeValueExistsAsync(fqn+"/value/1"),["value_missing"]=await sdk.AttributeValueExistsAsync(fqn+"/value/9")};
  h["listed"]=(await sdk.ListAttributesAsync(ns)).Select(a=>a.Fqn).ToArray();
  await sdk.ValidateAttributesAsync(new[]{fqn+"/value/1",fqn+"/value/2"});h["validate_ok"]=true;
  h["validate_missing"]=await ExpectFailure("validate missing",()=>sdk.ValidateAttributesAsync(new[]{fqn+"/value/1",fqn+"/value/9"}));
  h["entity_attributes"]=await sdk.GetEntityAttributesAsync(new OpenTDF.Protocol.Authorization.Entity{Id="e1",ClientId="opentdf-sdk"});
  var token=await sdk.Auth().AccessTokenAsync();
  h["access_token"]=new Dictionary<string,object>{["present"]=token.Value!="",["scheme"]=token.Scheme};
  await using(var over=await Sdk.CreateAsync("http://127.0.0.1:9",OptionsOf(cfg) with{CoreEndpoint=Text(cfg,"PlatformURL")}))
   h["override"]=new Dictionary<string,object>{["endpoint"]=(await over.ConnAsync()).Endpoint,["attribute_exists"]=await over.AttributeExistsAsync(fqn)};
  return h;
 }
 static async Task Repeat(Sdk sdk,JsonElement cfg)
 {
  byte[] original={0,255,128,1};byte[] archive=null;
  for(int i=0;i<100;i++)
  {
   byte[] input=original.Append((byte)i).ToArray();
   await using(var obj=await sdk.CreateTdfAsync(input,TdfOptionsOf(cfg) with{Metadata="round "+i,MimeType="application/日本語"}))
   {
    Check(await obj.SizeAsync()==obj.Bytes.Length,"size");
    var manifest=await obj.ManifestAsync();Check((string)((IReadOnlyDictionary<string,object>)manifest["payload"])["mimeType"]=="application/日本語","UTF-8 manifest");
    archive=obj.Bytes;
   }
   input[0]=9;
   await using(var reader=await sdk.LoadTdfAsync(archive,ReaderOptionsOf(cfg)))Check((await reader.DataAttributesAsync()).Count==1,"attributes");
   var plain=await sdk.DecryptBytesAsync(archive,ReaderOptionsOf(cfg));Check(plain.SequenceEqual(original.Append((byte)i)),"round trip");
  }
  var wrong=await ExpectFailure("wrong credentials",async()=>{await using var s=await Sdk.CreateAsync(Text(cfg,"PlatformURL"),OptionsOf(cfg) with{ClientSecret="wrong"});await s.DecryptBytesAsync(archive,ReaderOptionsOf(cfg));});
  Check((string)wrong["kind"]=="auth"&&(long)wrong["httpStatus"]==401,"wrong credentials rejected: "+JsonSerializer.Serialize(wrong));
  using var started=new ManualResetEventSlim();
  await using(var holder=await Sdk.CreateAsync(Text(cfg,"PlatformURL"),OptionsOf(cfg,false) with{TokenProvider=request=>{request.Retain();request.OnStop(()=>request.Reject("provider: stopped"));started.Set();}}))
  {
   using var cts=new CancellationTokenSource();
   var pending=holder.DecryptBytesAsync(archive,ReaderOptionsOf(cfg),cts.Token);
   Check(started.Wait(20000),"provider started");cts.Cancel();
   var canceled=await ExpectFailure("active cancel",()=>pending);Check((string)canceled["kind"]=="canceled","active provider cancel is typed: "+JsonSerializer.Serialize(canceled));
  }
  var asyncArchive=await sdk.EncryptBytesAsync(new byte[]{(byte)'a',0,255},TdfOptionsOf(cfg));
  Check((await sdk.DecryptBytesAsync(asyncArchive,ReaderOptionsOf(cfg))).SequenceEqual(new byte[]{(byte)'a',0,255}),"async round trip");
  await sdk.DisposeAsync();
  Check((string)(await ExpectFailure("closed",()=>sdk.DecryptBytesAsync(archive,ReaderOptionsOf(cfg))))["kind"]=="closed","closed handle");
 }

 /// One controlled case against the harness fixture (tests/interop/delivery/controlled_platform.py).
 static async Task Controlled(JsonElement cfg)
 {
  string url=Text(cfg,"PlatformURL"),kasPem=Text(cfg,"KASPublicKeyPEM");
  var offline=new TdfOptions{DisableAutoconfigure=true,KasInformation=new[]{new KasInfo("http://localhost:9999/malicious/kas"){PublicKey=kasPem,Kid="k1",Algorithm="rsa:2048"}},Attributes=new[]{"https://example.com/attr/a/value/b"}};
  var credentials=new Options{ClientId="opentdf",ClientSecret="secret"};
  Dictionary<string,object> error;
  switch(name)
  {
   case "unreachable":error=await ExpectFailure(name,()=>Sdk.CreateAsync(url,new Options{ValidateConnection=true}));break;
   case "caller-deadline":{using var cts=new CancellationTokenSource(1000);error=await ExpectFailure(name,()=>Sdk.CreateAsync(url,credentials,cts.Token));break;}
   case "source-deadline":error=await ExpectFailure(name,()=>Sdk.CreateAsync(url,credentials with{Timeout=TimeSpan.FromMilliseconds(100)}));break;
   case "token-401":
   {
    await using var sdk=await Sdk.CreateAsync(url,credentials);
    var archive=await sdk.EncryptBytesAsync(Encoding.UTF8.GetBytes("offline payload"),offline);
    error=await ExpectFailure(name,()=>sdk.DecryptBytesAsync(archive,new ReaderOptions{IgnoreAllowlist=true}));
    break;
   }
   case "offline":
   {
    var rows=new Dictionary<string,object>();
    var sdk=await Sdk.CreateAsync(url,credentials);
    var archive=await sdk.EncryptBytesAsync(Encoding.UTF8.GetBytes("offline payload"),offline);
    rows["allowlist"]=await ExpectFailure("allowlist",()=>sdk.DecryptBytesAsync(archive,new ReaderOptions{KasAllowlist=new[]{url}}));
    rows["invalid-tdf"]=await ExpectFailure("invalid-tdf",()=>sdk.DecryptBytesAsync(Encoding.UTF8.GetBytes("not a tdf")));
    rows["unsupported-option"]=await ExpectFailure("unsupported-option",()=>sdk.CreateTdfAsync(new byte[]{(byte)'x'},new TdfOptions{TargetMode="4.2.2"}));
    rows["invalid-argument"]=await ExpectFailure("invalid-argument",()=>sdk.CreateTdfAsync(null));
    for(int i=0;i<100;i++){await using var reader=await sdk.LoadTdfAsync(archive,new ReaderOptions{IgnoreAllowlist=true});Check((string)((IReadOnlyDictionary<string,object>)KeyAccess(await reader.ManifestAsync())[0])["kid"]=="k1","manifest");}
    await sdk.DisposeAsync();
    rows["closed-handle"]=await ExpectFailure("closed-handle",()=>sdk.DecryptBytesAsync(archive));
    rows["handles"]=new Dictionary<string,object>{["readers"]=100,["closed"]=true};
    Check(await Sdk.IsValidTdfAsync(archive)&&(await Sdk.VersionAsync())!=""&&(await Sdk.SupportedFeaturesAsync()).Contains("hexless"),"free functions");
    Write("offline.json",JsonSerializer.Serialize(rows));
    return;
   }
   default:error=await ExpectFailure(name,()=>Sdk.CreateAsync(url,credentials));break;
  }
  Write(name+".error.json",JsonSerializer.Serialize(error));
 }

 public static async Task<int> Main(string[] args)
 {
  if(args.Length!=4)throw new ArgumentException("consumer sdk run mode case");run=args[1];string mode=args[2];name=args[3];raw=JsonDocument.Parse(File.ReadAllText(Path.Combine(run,"config.json"))).RootElement;
  try
  {
   if(mode=="controlled"){await Controlled(raw);Console.WriteLine("PASS generated native library "+mode+" "+name);return 0;}
   await using var sdk=await Sdk.CreateAsync(Text(raw,"PlatformURL"),OptionsOf(raw));
   switch(mode)
   {
    case "encrypt":
    {
     var options=TdfOptionsOf(raw);
     if((name=="metadata"||name.EndsWith("-metadata"))&&name!="empty-metadata"&&!name.EndsWith("-empty-metadata"))options=options with{Metadata="{\"source\":\"independent metadata\",\"count\":7}"};
     if(name=="denied"||name.EndsWith("-denied"))options=options with{Attributes=new[]{"https://example.com/attr/attr1/value/value2"}};
     var split=name=="split"||name.EndsWith("-split");
     if(split)options=options with{KasInformation=new[]{KasInformation(raw),KasInformation(raw)}};
     await using var obj=await sdk.CreateTdfAsync(Read(name+".input"),options);
     Check(await obj.SizeAsync()==obj.Bytes.Length&&KeyAccess(await obj.ManifestAsync()).Count==(split?2:1),"object");
     Write(name+".generated.tdf",obj.Bytes);
     break;
    }
    case "decrypt":
    {
     var input=Read(name+".tdf");IReadOnlyDictionary<string,object> manifest;byte[] metadata=Array.Empty<byte>();bool hasMetadata;
     await using(var reader=await sdk.LoadTdfAsync(input,ReaderOptionsOf(raw)))
     {
      manifest=await reader.ManifestAsync();var kao=KeyAccess(manifest);
      hasMetadata=kao.Count>0&&(string)((IReadOnlyDictionary<string,object>)kao[0])["encryptedMetadata"]!="";
      if(hasMetadata){await reader.InitAsync();metadata=await reader.UnencryptedMetadataAsync();}
     }
     var payload=await sdk.DecryptBytesAsync(input,ReaderOptionsOf(raw));
     Write(name+".out",payload);Write(name+".metadata",metadata);Write(name+".manifest",JsonSerializer.Serialize(manifest));Write(name+".presence",hasMetadata?"true":"false");
     break;
    }
    case "negative":Write(name+".error.json",JsonSerializer.Serialize(await ExpectFailure("negative",()=>sdk.DecryptBytesAsync(Read(name+".tdf"),ReaderOptionsOf(raw)))));break;
    case "repeat":await Repeat(sdk,raw);break;
    case "services":await Services(sdk,raw);break;
    default:throw new ArgumentException("unknown mode");
   }
   Console.WriteLine("PASS generated native library "+mode+" "+name);
   return 0;
  }
  catch(OpenTdfException e){throw new Exception("consumer failure "+mode+" "+name+" "+JsonSerializer.Serialize(Report(e)),e);}
 }
}
