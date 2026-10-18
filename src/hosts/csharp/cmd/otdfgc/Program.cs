// otdfgc is the native .NET CLI over the generated assembly. It implements the
// xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc does:
// one SDK per invocation, created with --host and the client credentials;
// every flag this build cannot honour exits 2 as unsupported.
using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using OpenTDF;

#pragma warning disable CA1416
static class Program
{
    const int ExitSuccess = 0, ExitError = 1, ExitUsage = 2;
    const string Target = "csharp";
    static readonly TimeSpan Timeout = TimeSpan.FromSeconds(120);
    const string Usage = @"Usage:
  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--with-assertions <file|json>] [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--no-verify-assertions] [--with-assertion-verification-keys <file|json>] [--session-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc inspect <in>
  otdfgc supports <feature>
  otdfgc --version [--json]
";
    // The otdfctl flag subset of the CLI contract; boolean flags take no value.
    static readonly Dictionary<string,bool> Common=new(){["out"]=false,["host"]=false,["with-client-creds"]=false,["with-client-creds-file"]=false,["dpop"]=false,["dpop-key"]=false,["tls-no-verify"]=true,["log-level"]=false};
    static readonly Dictionary<string,bool> EncryptFlags=Merge(Common,new(){["attr"]=false,["mime-type"]=false,["with-assertions"]=false,["wrapping-key-algorithm"]=false,["target-mode"]=false,["policy-mode"]=false,["ecdsa-binding"]=true});
    static readonly Dictionary<string,bool> DecryptFlags=Merge(Common,new(){["with-assertion-verification-keys"]=false,["no-verify-assertions"]=true,["session-key-algorithm"]=false,["kas-allowlist"]=false});
    static readonly string[] DpopAlgorithms={"ES256","ES384","ES512","RS256","RS384","RS512"};
    // Rules naming the flags this build cannot honour, optionally only for values other than the allowed ones.
    static readonly (string,string[])[] EncryptUnsupported={("ecdsa-binding",null),("target-mode",new[]{"4.3.0"}),("policy-mode",new[]{"encrypted"}),("wrapping-key-algorithm",new[]{"rsa:2048","ec:secp256r1","rsa:4096","ec:secp384r1","ec:secp521r1"}),("dpop",DpopAlgorithms),("with-client-creds-file",null)};
    static readonly (string,string[])[] DecryptUnsupported={("session-key-algorithm",new[]{"rsa:2048","ec:secp256r1","rsa:4096","ec:secp384r1","ec:secp521r1"}),("dpop",DpopAlgorithms),("with-client-creds-file",null)};
    static Dictionary<string,bool> Merge(Dictionary<string,bool> a,Dictionary<string,bool> b){var m=new Dictionary<string,bool>(a);foreach(var e in b)m[e.Key]=e.Value;return m;}

    sealed class Parsed
    {
        public readonly Dictionary<string,string> Values=new();public readonly HashSet<string> Set=new();public readonly List<string> Positional=new();
        public string Value(string name)=>Values.TryGetValue(name,out var v)?v:"";public bool Flag(string name)=>Set.Contains(name);
    }
    static Parsed Parse(string[] args,Dictionary<string,bool> spec)
    {
        var p=new Parsed();
        for(int i=0;i<args.Length;i++)
        {
            string a=args[i];
            if(!a.StartsWith("-")||a=="-"){p.Positional.Add(a);continue;}
            string name=a.TrimStart('-'),value="";bool inline=false;
            int eq=name.IndexOf('=');if(eq>=0){value=name[(eq+1)..];name=name[..eq];inline=true;}
            if(name=="o")name="out";
            if(!spec.TryGetValue(name,out var isBool))throw new ArgumentException("unknown flag: "+a);
            if(isBool){if(inline&&value!="true"&&value!="false")throw new ArgumentException("flag --"+name+" takes no value");if(!inline||value=="true")p.Set.Add(name);continue;}
            if(!inline){if(i+1>=args.Length)throw new ArgumentException("flag --"+name+" needs a value");value=args[++i];}
            p.Values[name]=value;p.Set.Add(name);
        }
        return p;
    }
    static string Unsupported(Parsed p,(string,string[])[] rules)
    {
        foreach(var (name,allowed) in rules){if(!p.Flag(name))continue;if(allowed==null)return "--"+name;if(!allowed.Contains(p.Value(name)))return "--"+name+" "+p.Value(name);}
        return "";
    }
    static List<string> SplitList(string s)=>s.Split(',').Select(x=>x.Trim()).Where(x=>x!="").ToList();

    // Describe renders an SDK failure: the reference SDK's wording plus otdfctl's "required obligations" suffix.
    static string Describe(Exception e)
    {
        if(e is not OpenTdfException o)return e.Message;
        string msg=o.Message;if(o.RequiredObligations.Count>0)msg+="\nrequired obligations: ["+string.Join(" ",o.RequiredObligations)+"]";return msg;
    }
    static string DescribeNew(Exception e,string host)
    {
        if(e is not OpenTdfException o)return e.Message;
        if(o.Kind=="auth")return "Failed to authenticate with flag-provided client credentials. ("+o.Message+")";
        if(o.Message.Contains("platform unreachable or not responding")||o.Kind=="transport"||o.Kind=="deadline_exceeded")return "Failed to connect to the platform. Is the platform accepting connections at '"+host+"'? ("+o.Message+")";
        return o.Message;
    }
    static async Task<T> Run<T>(Func<CancellationToken,Task<T>> call)
    {
        using var cts=new CancellationTokenSource(Timeout);
        try{return await call(cts.Token);}
        catch(Exception e)when(e is not OpenTdfException&&cts.IsCancellationRequested){throw new TimeoutException("deadline exceeded after "+Timeout,e);}
    }

    // Builds the SDK from --host, --with-client-creds and the DPoP and TLS flags; the SDK discovers everything else.
    static async Task<(Sdk,string,int)> NewSdk(Parsed p,TextWriter stderr)
    {
        string host=p.Value("host").TrimEnd('/');
        if(host=="") {stderr.Write("--host is required\n"+Usage);return (null,host,ExitUsage);}
        string creds=p.Value("with-client-creds");
        if(creds=="") {stderr.Write("--with-client-creds is required\n"+Usage);return (null,host,ExitUsage);}
        string id=null,secret=null;
        try{using var doc=JsonDocument.Parse(creds);if(doc.RootElement.ValueKind==JsonValueKind.Object){if(doc.RootElement.TryGetProperty("clientId",out var i)&&i.ValueKind==JsonValueKind.String)id=i.GetString();if(doc.RootElement.TryGetProperty("clientSecret",out var s)&&s.ValueKind==JsonValueKind.String)secret=s.GetString();}}catch(JsonException){}
        if(string.IsNullOrEmpty(id)||string.IsNullOrEmpty(secret)){stderr.WriteLine("--with-client-creds must be JSON with clientId and clientSecret");return (null,host,ExitUsage);}
        var options=new Options{ClientId=id,ClientSecret=secret,InsecurePlaintext=host.ToLowerInvariant().StartsWith("http://"),InsecureSkipVerify=p.Flag("tls-no-verify"),DpopAlgorithm=p.Value("dpop")==""?null:p.Value("dpop")};
        if(p.Value("dpop-key")!="")
        {
            if(p.Value("dpop")==""){stderr.WriteLine("--dpop-key needs --dpop");return (null,host,ExitUsage);}
            try{options=options with{DpopKeyPem=File.ReadAllText(p.Value("dpop-key"))};}
            catch(IOException e){stderr.WriteLine("Failed to read DPoP key "+p.Value("dpop-key")+": "+e.Message);return (null,host,ExitError);}
        }
        try{return (await Run(ct=>Sdk.CreateAsync(host,options,ct)),host,ExitSuccess);}
        catch(Exception e)when(e is OpenTdfException||e is TimeoutException){stderr.WriteLine(DescribeNew(e,host));return (null,host,ExitError);}
    }
    static void WriteOut(string path,byte[] data,UnixFileMode mode)
    {
        string tmp=path+".tmp";File.WriteAllBytes(tmp,data);
        try{File.SetUnixFileMode(tmp,mode);}catch(PlatformNotSupportedException){}
        try{File.Move(tmp,path,true);}catch(IOException){File.Delete(tmp);throw;}
    }

    static async Task<int> Version(string[] args,TextWriter stdout,TextWriter stderr)
    {
        bool asJson=false;
        foreach(var a in args){if(a=="--json")asJson=true;else{stderr.WriteLine("unknown flag: "+a);return ExitUsage;}}
        string version,schema;IReadOnlyList<string> features;
        try{version=await Sdk.VersionAsync();schema=await Sdk.TdfSpecVersionAsync();features=await Sdk.SupportedFeaturesAsync();}
        catch(OpenTdfException e){stderr.WriteLine("Failed to read version: "+Describe(e));return ExitError;}
        if(asJson){stdout.WriteLine(JsonSerializer.Serialize(new Dictionary<string,object>{["sdk_version"]=version,["schema_version"]=schema,["supported_features"]=features,["target"]=Target}));return ExitSuccess;}
        stdout.WriteLine("otdfgc ("+Target+") sdk "+version+" tdf-spec "+schema+" features "+string.Join(",",features));
        return ExitSuccess;
    }
    static async Task<int> Supports(string[] args,TextWriter stdout,TextWriter stderr)
    {
        if(args.Length!=1||args[0].StartsWith("-")){stderr.WriteLine("usage: otdfgc supports <feature>");return ExitUsage;}
        IReadOnlyList<string> features;
        try{features=await Sdk.SupportedFeaturesAsync();}catch(OpenTdfException e){stderr.WriteLine("Failed to read supported features: "+Describe(e));return ExitError;}
        if(features.Contains(args[0]))return ExitSuccess;
        stdout.WriteLine(args[0]+" unsupported");return ExitError;
    }
    static byte[] ReadManifest(string path)
    {
        using var archive=ZipFile.OpenRead(path);
        var entry=archive.GetEntry("0.manifest.json")??throw new InvalidDataException("manifest entry not found");
        using var stream=entry.Open();using var memory=new MemoryStream();stream.CopyTo(memory);return memory.ToArray();
    }
    static int Inspect(string[] args,TextWriter stdout,TextWriter stderr)
    {
        if(args.Length!=1||args[0].StartsWith("-")){stderr.WriteLine("usage: otdfgc inspect <in>");return ExitUsage;}
        byte[] manifest;
        try{manifest=ReadManifest(args[0]);}catch(Exception e)when(e is IOException||e is InvalidDataException){stderr.WriteLine("not a valid TDF: "+e.Message);return ExitError;}
        try{using var doc=JsonDocument.Parse(manifest);stdout.WriteLine(JsonSerializer.Serialize(doc.RootElement,new JsonSerializerOptions{WriteIndented=true,Encoder=System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping}));}
        catch(JsonException e){stderr.WriteLine("failed to inspect TDF: "+e.Message);return ExitError;}
        return ExitSuccess;
    }
    // --with-assertions and --with-assertion-verification-keys take a file path or JSON text, as otdfctl does:
    // an object statement value is kept as compact JSON with sorted keys; HS256 keys are the string's bytes, RS256 keys PEM.
    static string FileOrJson(string arg)
    {
        try{JsonDocument.Parse(arg).Dispose();return arg;}catch(JsonException){}
        try{return File.ReadAllText(arg);}
        catch(Exception e)when(e is IOException||e is UnauthorizedAccessException){throw new IOException(arg+" is neither JSON nor a readable file: "+e.Message);}
    }
    static string Text(JsonElement e,string key)=>e.ValueKind==JsonValueKind.Object&&e.TryGetProperty(key,out var v)&&v.ValueKind==JsonValueKind.String?v.GetString():"";
    static void WriteSorted(Utf8JsonWriter w,JsonElement e)
    {
        switch(e.ValueKind)
        {
            case JsonValueKind.Object:w.WriteStartObject();foreach(var property in e.EnumerateObject().OrderBy(x=>x.Name,StringComparer.Ordinal)){w.WritePropertyName(property.Name);WriteSorted(w,property.Value);}w.WriteEndObject();break;
            case JsonValueKind.Array:w.WriteStartArray();foreach(var item in e.EnumerateArray())WriteSorted(w,item);w.WriteEndArray();break;
            default:e.WriteTo(w);break;
        }
    }
    static string Compact(JsonElement e)
    {
        using var stream=new MemoryStream();
        using(var w=new Utf8JsonWriter(stream,new JsonWriterOptions{Encoder=System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping}))WriteSorted(w,e);
        return Encoding.UTF8.GetString(stream.ToArray());
    }
    static AssertionKey AssertionKeyOf(JsonElement e,string label)
    {
        string alg=Text(e,"alg");
        if(alg!="HS256"&&alg!="RS256")throw new ArgumentException(label+": unsupported signing key algorithm "+alg);
        return new AssertionKey(alg,Text(e,"key"));
    }
    static IReadOnlyList<AssertionConfig> ParseAssertions(string arg)
    {
        using var doc=JsonDocument.Parse(FileOrJson(arg));
        if(doc.RootElement.ValueKind!=JsonValueKind.Array)throw new ArgumentException("assertions must be a JSON list");
        var configs=new List<AssertionConfig>();
        foreach(var a in doc.RootElement.EnumerateArray())
        {
            if(a.ValueKind!=JsonValueKind.Object)throw new ArgumentException("assertions must be a JSON list of objects");
            string id=Text(a,"id");
            JsonElement statement=a.TryGetProperty("statement",out var st)&&st.ValueKind==JsonValueKind.Object?st:default;
            string value="";
            if(statement.ValueKind==JsonValueKind.Object&&statement.TryGetProperty("value",out var v)&&v.ValueKind!=JsonValueKind.Null)
            {
                if(v.ValueKind==JsonValueKind.String)value=v.GetString();
                else if(v.ValueKind==JsonValueKind.Object)value=Compact(v);
                else throw new ArgumentException("assertion "+id+": statement value is neither a string nor an object");
            }
            var config=new AssertionConfig(id,Text(a,"type"),Text(a,"scope"),Text(a,"appliesToState"),Text(statement,"format"),Text(statement,"schema"),value);
            if(a.TryGetProperty("signingKey",out var key)&&key.ValueKind==JsonValueKind.Object)config=config with{SigningKey=AssertionKeyOf(key,"assertion "+id)};
            configs.Add(config);
        }
        return configs;
    }
    static AssertionVerificationKeys ParseVerificationKeys(string arg)
    {
        using var doc=JsonDocument.Parse(FileOrJson(arg));
        if(doc.RootElement.ValueKind!=JsonValueKind.Object)throw new ArgumentException("assertion verification keys must be a JSON object");
        var keys=new AssertionVerificationKeys();
        if(doc.RootElement.TryGetProperty("defaultKey",out var d)&&d.ValueKind==JsonValueKind.Object)keys=keys with{DefaultKey=new AssertionKey(Text(d,"alg"),Text(d,"key"))};
        if(doc.RootElement.TryGetProperty("keys",out var k)&&k.ValueKind==JsonValueKind.Object)
            keys=keys with{Keys=k.EnumerateObject().ToDictionary(e=>e.Name,e=>new AssertionKey(Text(e.Value,"alg"),Text(e.Value,"key")))};
        return keys;
    }
    static async Task<int> Encrypt(string[] args,TextWriter stderr)
    {
        Parsed p;try{p=Parse(args,EncryptFlags);}catch(ArgumentException e){stderr.Write(e.Message+"\n"+Usage);return ExitUsage;}
        string reason=Unsupported(p,EncryptUnsupported);if(reason!=""){stderr.WriteLine("unsupported in this build: "+reason);return ExitUsage;}
        if(p.Positional.Count!=1||p.Value("out")==""){stderr.Write("encrypt needs exactly one input file and --out\n"+Usage);return ExitUsage;}
        byte[] input;try{input=File.ReadAllBytes(p.Positional[0]);}catch(IOException e){stderr.WriteLine("Failed to read "+p.Positional[0]+": "+e.Message);return ExitError;}
        var (sdk,host,code)=await NewSdk(p,stderr);if(code!=ExitSuccess)return code;
        await using(sdk)
        {
            // As otdfctl: the platform's KAS is the default key access server and attributes drive autoconfigure.
            var options=new TdfOptions{KasInformation=new[]{new KasInfo(host+"/kas")},MimeType=p.Value("mime-type")};
            if(p.Value("attr")!="")options=options with{Attributes=SplitList(p.Value("attr"))};
            if(p.Value("wrapping-key-algorithm")!="")options=options with{WrappingKeyAlgorithm=p.Value("wrapping-key-algorithm")};
            if(p.Value("target-mode")!="")options=options with{TargetMode=p.Value("target-mode")};
            if(p.Value("with-assertions")!="")
            {
                try{options=options with{Assertions=ParseAssertions(p.Value("with-assertions"))};}
                catch(Exception e)when(e is ArgumentException||e is IOException||e is JsonException){stderr.WriteLine("Failed to read assertions: "+e.Message);return ExitUsage;}
            }
            byte[] data;
            try{data=await Run(ct=>sdk.EncryptBytesAsync(input,options,ct));}
            catch(Exception e)when(e is OpenTdfException||e is TimeoutException){stderr.WriteLine("Failed to encrypt: "+Describe(e));return ExitError;}
            try{WriteOut(p.Value("out"),data,UnixFileMode.UserRead|UnixFileMode.UserWrite|UnixFileMode.GroupRead|UnixFileMode.OtherRead);}
            catch(IOException e){stderr.WriteLine("Failed to write encrypted file "+p.Value("out")+": "+e.Message);return ExitError;}
        }
        return ExitSuccess;
    }
    static async Task<int> Decrypt(string[] args,TextWriter stderr)
    {
        Parsed p;try{p=Parse(args,DecryptFlags);}catch(ArgumentException e){stderr.Write(e.Message+"\n"+Usage);return ExitUsage;}
        string reason=Unsupported(p,DecryptUnsupported);if(reason!=""){stderr.WriteLine("unsupported in this build: "+reason);return ExitUsage;}
        if(p.Positional.Count!=1||p.Value("out")==""){stderr.Write("decrypt needs exactly one input file and --out\n"+Usage);return ExitUsage;}
        byte[] input;try{input=File.ReadAllBytes(p.Positional[0]);}catch(IOException e){stderr.WriteLine("Failed to read file: "+e.Message);return ExitError;}
        var (sdk,_,code)=await NewSdk(p,stderr);if(code!=ExitSuccess)return code;
        await using(sdk)
        {
            var options=new ReaderOptions{DisableAssertionVerification=p.Flag("no-verify-assertions")};
            if(p.Value("with-assertion-verification-keys")!="")
            {
                try{options=options with{AssertionVerificationKeys=ParseVerificationKeys(p.Value("with-assertion-verification-keys"))};}
                catch(Exception e)when(e is ArgumentException||e is IOException||e is JsonException){stderr.WriteLine("Failed to read assertion verification keys: "+e.Message);return ExitUsage;}
            }
            if(p.Value("session-key-algorithm")!="")options=options with{SessionKeyType=p.Value("session-key-algorithm")};
            // otdfctl: "*" alone ignores the allowlist; any other list is explicit.
            var allowlist=SplitList(p.Value("kas-allowlist"));
            if(allowlist.Count==1&&allowlist[0]=="*")options=options with{IgnoreAllowlist=true};
            else if(allowlist.Count>0)options=options with{KasAllowlist=allowlist};
            byte[] plaintext;
            try{plaintext=await Run(ct=>sdk.DecryptBytesAsync(input,options,ct));}
            catch(Exception e)when(e is OpenTdfException||e is TimeoutException){stderr.WriteLine("Failed to decrypt file: "+Describe(e));return ExitError;}
            try{WriteOut(p.Value("out"),plaintext,UnixFileMode.UserRead|UnixFileMode.UserWrite);}
            catch(IOException e){stderr.WriteLine("Failed to write decrypted data to file: "+e.Message);return ExitError;}
        }
        return ExitSuccess;
    }
    static async Task<int> Main(string[] args)
    {
        var stdout=new StreamWriter(Console.OpenStandardOutput(),new UTF8Encoding(false)){AutoFlush=true};var stderr=new StreamWriter(Console.OpenStandardError(),new UTF8Encoding(false)){AutoFlush=true};
        if(args.Length==0){stderr.Write(Usage);return ExitUsage;}
        string command=args[0];var rest=args[1..];
        switch(command)
        {
            case "--version":case "version":return await Version(rest,stdout,stderr);
            case "supports":return await Supports(rest,stdout,stderr);
            case "inspect":return Inspect(rest,stdout,stderr);
            case "encrypt":return await Encrypt(rest,stderr);
            case "decrypt":return await Decrypt(rest,stderr);
            case "help":case "--help":case "-h":stdout.Write(Usage);return ExitSuccess;
            default:stderr.Write("unknown command: "+command+"\n"+Usage);return ExitUsage;
        }
    }
}
