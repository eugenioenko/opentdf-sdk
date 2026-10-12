package consumer;

import io.opentdf.tdf3.OpenTDFException;
import io.opentdf.tdf3.Reader;
import io.opentdf.tdf3.ReaderOptions;
import io.opentdf.tdf3.SDK;
import io.opentdf.tdf3.TDF3;
import io.opentdf.tdf3.TDFObject;
import io.opentdf.tdf3.TDFOptions;
import java.nio.file.*;
import java.nio.charset.StandardCharsets;
import java.security.*;
import java.security.spec.*;
import java.security.interfaces.*;
import java.net.*;
import java.net.http.*;
import java.time.*;
import java.util.*;
import java.util.concurrent.*;
import org.bouncycastle.asn1.sec.SECNamedCurves;
import org.bouncycastle.math.ec.FixedPointCombMultiplier;

/** Independent importing consumer of the handle API: one SDK per run, createTDF/decryptBytes, loadTDF readers, 100 round trips, and the controlled checks of tests/interop/delivery/controlled_platform.py. Host OAuth uses native JCA/HTTP. */
public final class Consumer {
 static Path run;static Map<String,Object> raw;static String name;
 static final String ATTRIBUTE="https://example.com/attr/attr1/value/value1";
 static String text(Map<String,Object> o,String k){return o.get(k) instanceof String s?s:"";}
 static boolean bool(Map<String,Object> o,String k){return Boolean.TRUE.equals(o.get(k));}
 static void check(boolean ok,String label){if(!ok)throw new AssertionError(label);}
 static byte[] read(String file)throws Exception{return Files.readAllBytes(run.resolve(file));}
 static void write(String file,byte[] bytes)throws Exception{Path p=run.resolve(file);Files.write(p,bytes);Files.setPosixFilePermissions(p,java.nio.file.attribute.PosixFilePermissions.fromString("rw-------"));}
 static void write(String file,String value)throws Exception{write(file,value.getBytes(StandardCharsets.UTF_8));}
 static String b64(byte[] b){return Base64.getUrlEncoder().withoutPadding().encodeToString(b);}
 static byte[] unsigned(java.math.BigInteger n){byte[] b=n.toByteArray();return b.length>1&&b[0]==0?Arrays.copyOfRange(b,1,b.length):b;}
 static byte[] coordinate(java.math.BigInteger n){byte[] b=unsigned(n),r=new byte[32];System.arraycopy(b,0,r,32-b.length,b.length);return r;}
 static PrivateKey privateKey(String pem)throws Exception{String encoded=pem.replaceAll("-----[^-]+-----","").replaceAll("\\s","");byte[] der=Base64.getDecoder().decode(encoded);try{return KeyFactory.getInstance("RSA").generatePrivate(new PKCS8EncodedKeySpec(der));}catch(InvalidKeySpecException e){return KeyFactory.getInstance("EC").generatePrivate(new PKCS8EncodedKeySpec(der));}}
 static Map<String,Object> jwk(PrivateKey key){Map<String,Object> m=new TreeMap<>();if(key instanceof RSAPrivateCrtKey k){m.put("e",b64(unsigned(k.getPublicExponent())));m.put("kty","RSA");m.put("n",b64(unsigned(k.getModulus())));}else{ECPrivateKey k=(ECPrivateKey)key;var q=new FixedPointCombMultiplier().multiply(SECNamedCurves.getByName("secp256r1").getG(),k.getS()).normalize();m.put("crv","P-256");m.put("kty","EC");m.put("x",b64(coordinate(q.getAffineXCoord().toBigInteger())));m.put("y",b64(coordinate(q.getAffineYCoord().toBigInteger())));}return m;}
 static String proof(PrivateKey key,Map<String,Object> jwk,String endpoint,String nonce)throws Exception{
  Map<String,Object> header=new LinkedHashMap<>();header.put("typ","dpop+jwt");header.put("alg",key instanceof RSAPrivateKey?"RS256":"ES256");header.put("jwk",jwk);
  byte[] random=new byte[16];new SecureRandom().nextBytes(random);Map<String,Object> claims=new LinkedHashMap<>();claims.put("htu",endpoint);claims.put("htm","POST");claims.put("iat",Instant.now().getEpochSecond());claims.put("jti",b64(random));if(!nonce.isEmpty())claims.put("nonce",nonce);
  String unsigned=b64(Json.write(header).getBytes(StandardCharsets.UTF_8))+"."+b64(Json.write(claims).getBytes(StandardCharsets.UTF_8));Signature sign=Signature.getInstance(key instanceof RSAPrivateKey?"SHA256withRSA":"SHA256withECDSAinP1363Format");sign.initSign(key);sign.update(unsigned.getBytes(StandardCharsets.US_ASCII));return unsigned+"."+b64(sign.sign());
 }
 // The host token source: native OAuth client credentials with a DPoP proof when the profile asks for one.
 @SuppressWarnings("unchecked")static TDF3.AccessToken token(Map<String,Object> cfg,TDF3.ProviderRequest request)throws Exception{
  long now=Instant.now().getEpochSecond();if(name.equals("invalid-token"))return new TDF3.AccessToken("invalid-token","Bearer",now+300,"");if(name.equals("expired-token"))return new TDF3.AccessToken("expired-token","Bearer",now-1,"");if(name.equals("provider-reject"))throw new IllegalArgumentException("provider rejected");
  String endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token",nonce="",thumb="";PrivateKey key=null;Map<String,Object> jwk=null;boolean dpop=bool(cfg,"DPoP");
  if(dpop){if(name.equals("mismatched-provider")){KeyPairGenerator generator=KeyPairGenerator.getInstance("EC");generator.initialize(new ECGenParameterSpec("secp256r1"));key=generator.generateKeyPair().getPrivate();}else key=privateKey(text(cfg,"AuthPrivateKeyPEM"));jwk=jwk(key);thumb=b64(MessageDigest.getInstance("SHA-256").digest(Json.write(jwk).getBytes(StandardCharsets.UTF_8)));}
  Thread thread=Thread.currentThread();
  try(HttpClient client=HttpClient.newBuilder().followRedirects(HttpClient.Redirect.NEVER).connectTimeout(Duration.ofSeconds(10)).build()){
   request.onStop(()->{thread.interrupt();client.shutdownNow();});
   for(int attempt=0;attempt<3;attempt++){
    HttpRequest.Builder builder=HttpRequest.newBuilder(URI.create(endpoint)).timeout(Duration.ofSeconds(15)).header("Content-Type","application/x-www-form-urlencoded").POST(HttpRequest.BodyPublishers.ofString("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret"));if(dpop)builder.header("DPoP",proof(key,jwk,endpoint,nonce));HttpResponse<byte[]> response=client.send(builder.build(),HttpResponse.BodyHandlers.ofByteArray());
    if(response.body().length>128<<10)throw new IllegalArgumentException("provider body limit");String next=response.headers().firstValue("DPoP-Nonce").orElse("");if((response.statusCode()==400||response.statusCode()==401)&&!next.isEmpty()&&!next.equals(nonce)){nonce=next;continue;}if(response.statusCode()!=200)throw new IllegalArgumentException("provider status");Map<String,Object> value=(Map<String,Object>)Json.parse(new String(response.body(),StandardCharsets.UTF_8));return new TDF3.AccessToken(text(value,"access_token"),dpop?"DPoP":"Bearer",Instant.now().getEpochSecond()+Long.parseLong(text(value,"expires_in")),thumb);
   }
  }
  throw new IllegalArgumentException("provider nonce retries");
 }
 // config.json carries the façade Config fields the delivery scripts write; they map onto the handle API's options as the Go consumer maps them.
 static SDK.Builder builder(Map<String,Object> cfg){
  SDK.Builder b=SDK.builder(text(cfg,"PlatformURL")).insecurePlaintext(bool(cfg,"AllowHTTP"));
  if(!text(cfg,"TokenURL").isEmpty())b.tokenEndpoint(text(cfg,"TokenURL"));
  if(!text(cfg,"ClientID").isEmpty())b.clientCredentials(text(cfg,"ClientID"),text(cfg,"ClientSecret"));
  if(!text(cfg,"TokenProviderName").isEmpty())b.tokenProvider(request->{request.retain();Thread.ofVirtual().start(()->{try{request.resolve(token(cfg,request));}catch(Exception e){request.reject("provider: rejected");}});});
  if(cfg.get("TimeoutMillis")!=null)b.timeout(Duration.ofMillis(Long.parseLong(String.valueOf(cfg.get("TimeoutMillis")))));
  if(bool(cfg,"DPoP"))b.dpopAlgorithm(text(cfg,"AuthAlgorithm").isEmpty()?"ES256":text(cfg,"AuthAlgorithm"));
  if(!text(cfg,"AuthPrivateKeyPEM").isEmpty())b.dpopKeyPem(text(cfg,"AuthPrivateKeyPEM"));
  return b;
 }
 static TDFOptions.KASInfo kasInformation(Map<String,Object> cfg){String url=text(cfg,"KASURL").isEmpty()?text(cfg,"PlatformURL").replaceAll("/+$","")+"/kas":text(cfg,"KASURL");return new TDFOptions.KASInfo(url,text(cfg,"KASPublicKeyPEM"),text(cfg,"KID"),text(cfg,"KASAlgorithm"),false);}
 static TDFOptions.Builder tdfOptions(Map<String,Object> cfg){TDFOptions.Builder o=TDFOptions.builder().disableAutoconfigure(true).kasInformation(kasInformation(cfg)).attributes(ATTRIBUTE).segmentSize(16384);if(!text(cfg,"KASAlgorithm").isEmpty())o.wrappingKeyAlgorithm(text(cfg,"KASAlgorithm"));return o;}
 @SuppressWarnings("unchecked")static ReaderOptions readerOptions(Map<String,Object> cfg){
  List<String> allow=new ArrayList<>();allow.add(text(cfg,"PlatformURL"));if(!text(cfg,"KASURL").isEmpty())allow.add(text(cfg,"KASURL"));
  if(cfg.get("AllowedKAS") instanceof List<?> routes)for(Object r:routes)allow.add(text((Map<String,Object>)r,"URL"));
  ReaderOptions.Builder o=ReaderOptions.builder().kasAllowlist(allow);if(!text(cfg,"SessionAlgorithm").isEmpty())o.sessionKeyType(text(cfg,"SessionAlgorithm"));return o.build();
 }
 static Map<String,Object> report(Exception e){
  Map<String,Object> m=new LinkedHashMap<>();
  if(e instanceof OpenTDFException o){m.put("kind",o.kind());m.put("op",o.op());m.put("httpStatus",o.httpStatus());m.put("connectCode",o.connectCode());m.put("message",o.getMessage());}
  else m.put("untyped",e.getClass().getName()+": "+e.getMessage());
  return m;
 }
 static Map<String,Object> expectFailure(String label,Callable<?> call){try{call.call();throw new AssertionError(label+" returned successful output");}catch(OpenTDFException e){return report(e);}catch(AssertionError e){throw e;}catch(Exception e){throw new AssertionError(label+" untyped: "+e,e);}}

 /** 100 encrypt and decrypt calls on one SDK, independent inputs and results, every handle closed; then the SDK is closed and later calls fail. */
 @SuppressWarnings("unchecked")static void repeat(SDK sdk,Map<String,Object> cfg)throws Exception{
  byte[] original={0,(byte)255,(byte)128,1};
  byte[] archive=null;
  for(int i=0;i<100;i++){
   byte[] input=Arrays.copyOf(original,5);input[4]=(byte)i;
   try(TDFObject object=sdk.createTDF(input,tdfOptions(cfg).metadata("round "+i).mimeType("application/日本語").build())){
    check(object.size()==object.bytes().length,"size");
    Map<String,Object> manifest=object.manifest();check("application/日本語".equals(((Map<String,Object>)manifest.get("payload")).get("mimeType")),"UTF-8 manifest");
    archive=object.bytes();
   }
   input[0]=9;
   try(Reader reader=sdk.loadTDF(archive,readerOptions(cfg))){check(reader.dataAttributes().size()==1,"attributes");}
   byte[] plain=sdk.decryptBytes(archive,readerOptions(cfg));
   byte[] expected=Arrays.copyOf(original,5);expected[4]=(byte)i;check(Arrays.equals(plain,expected),"round trip");
  }
  final byte[] last=archive;
  Map<String,Object> wrong=expectFailure("wrong credentials",()->{try(SDK s=builder(cfg).clientCredentials(text(cfg,"ClientID"),"wrong").build()){return s.decryptBytes(last,readerOptions(cfg));}});
  check("auth".equals(wrong.get("kind"))&&Long.valueOf(401).equals(wrong.get("httpStatus")),"wrong credentials rejected: "+wrong);
  CountDownLatch started=new CountDownLatch(1);
  Map<String,Object> provider=new LinkedHashMap<>(cfg);provider.remove("ClientID");provider.remove("ClientSecret");
  try(SDK holder=builder(provider).tokenProvider(request->{request.retain();request.onStop(()->request.reject("provider: stopped"));started.countDown();}).build()){
   CompletableFuture<byte[]> pending=holder.decryptBytesAsync(last,readerOptions(cfg));
   check(started.await(20,TimeUnit.SECONDS),"provider started");
   pending.cancel(true);
   try{pending.get(20,TimeUnit.SECONDS);throw new AssertionError("canceled call returned");}catch(CancellationException|ExecutionException expected){}
   Map<String,Object> timed=expectFailure("deadline",()->holder.decryptBytes(last,readerOptions(cfg),Duration.ofMillis(500)));
   check("deadline_exceeded".equals(timed.get("kind")),"blocking deadline: "+timed);
  }
  byte[] async=sdk.createTDFAsync(new byte[]{'a',0,(byte)255},tdfOptions(cfg).build()).thenCompose(o->o.closeAsync().thenApply(v->o.bytes())).get(60,TimeUnit.SECONDS);
  check(Arrays.equals(sdk.decryptBytesAsync(async,readerOptions(cfg)).get(60,TimeUnit.SECONDS),new byte[]{'a',0,(byte)255}),"async round trip");
  sdk.close();
  check("closed".equals(expectFailure("closed",()->sdk.decryptBytes(last,readerOptions(cfg))).get("kind")),"closed handle");
 }

 /** One controlled case against the harness fixture (tests/interop/delivery/controlled_platform.py). */
 @SuppressWarnings("unchecked")static void controlled(Map<String,Object> cfg)throws Exception{
  String url=text(cfg,"PlatformURL"),kasPem=text(cfg,"KASPublicKeyPEM");
  TDFOptions offline=TDFOptions.builder().disableAutoconfigure(true).kasInformation(new TDFOptions.KASInfo("http://localhost:9999/malicious/kas",kasPem,"k1","rsa:2048",false)).attributes("https://example.com/attr/a/value/b").build();
  Map<String,Object> error;
  switch(name){
   case "unreachable"->error=expectFailure(name,()->SDK.builder(url).validateConnection(true).build());
   case "caller-deadline"->error=expectFailure(name,()->SDK.builder(url).clientCredentials("opentdf","secret").build(Duration.ofSeconds(1)));
   case "source-deadline"->error=expectFailure(name,()->SDK.builder(url).clientCredentials("opentdf","secret").timeout(Duration.ofMillis(100)).build());
   case "token-401"->{
    try(SDK sdk=SDK.builder(url).clientCredentials("opentdf","secret").build()){
     byte[] archive=sdk.encryptBytes("offline payload".getBytes(StandardCharsets.UTF_8),offline);
     error=expectFailure(name,()->sdk.decryptBytes(archive,ReaderOptions.builder().ignoreAllowlist(true).build()));
    }
   }
   case "offline"->{
    Map<String,Object> rows=new LinkedHashMap<>();
    SDK sdk=SDK.builder(url).clientCredentials("opentdf","secret").build();
    byte[] archive=sdk.encryptBytes("offline payload".getBytes(StandardCharsets.UTF_8),offline);
    rows.put("allowlist",expectFailure("allowlist",()->sdk.decryptBytes(archive,ReaderOptions.builder().kasAllowlist(url).build())));
    rows.put("invalid-tdf",expectFailure("invalid-tdf",()->sdk.decryptBytes("not a tdf".getBytes(StandardCharsets.UTF_8),ReaderOptions.defaults())));
    rows.put("unsupported-option",expectFailure("unsupported-option",()->sdk.createTDF(new byte[]{'x'},TDFOptions.builder().targetMode("4.2.2").build())));
    rows.put("invalid-argument",expectFailure("invalid-argument",()->sdk.createTDF(null,TDFOptions.builder().build())));
    for(int i=0;i<100;i++){try(Reader reader=sdk.loadTDF(archive,ReaderOptions.builder().ignoreAllowlist(true).build())){Map<String,Object> manifest=reader.manifest();check("k1".equals(((Map<String,Object>)((List<Object>)((Map<String,Object>)manifest.get("encryptionInformation")).get("keyAccess")).get(0)).get("kid")),"manifest");}}
    sdk.close();
    rows.put("closed-handle",expectFailure("closed-handle",()->sdk.decryptBytes(archive,ReaderOptions.defaults())));
    Map<String,Object> handles=new LinkedHashMap<>();handles.put("readers",100);handles.put("closed",true);rows.put("handles",handles);
    check(SDK.isValidTdf(archive)&&!SDK.version().isEmpty()&&SDK.supportedFeatures().contains("hexless"),"free functions");
    write("offline.json",Json.write(rows));
    return;
   }
   default->error=expectFailure(name,()->SDK.builder(url).clientCredentials("opentdf","secret").build());
  }
  write(name+".error.json",Json.write(error));
 }

 @SuppressWarnings("unchecked")public static void main(String[] args)throws Exception{
  if(args.length!=4)throw new IllegalArgumentException("consumer sdk run mode case");run=Path.of(args[1]);String mode=args[2];name=args[3];raw=(Map<String,Object>)Json.parse(Files.readString(run.resolve("config.json")));
  try{
   if(mode.equals("controlled")){controlled(raw);System.out.println("PASS generated native library "+mode+" "+name);return;}
   try(SDK sdk=builder(raw).build()){
    switch(mode){
     case "encrypt"->{
      TDFOptions.Builder options=tdfOptions(raw);
      if((name.equals("metadata")||name.endsWith("-metadata"))&&!name.equals("empty-metadata")&&!name.endsWith("-empty-metadata"))options.metadata("{\"source\":\"independent metadata\",\"count\":7}");
      if(name.equals("denied")||name.endsWith("-denied"))options.attributes("https://example.com/attr/attr1/value/value2");
      try(TDFObject object=sdk.createTDF(read(name+".input"),options.build())){
       check(object.size()==object.bytes().length&&((List<Object>)((Map<String,Object>)object.manifest().get("encryptionInformation")).get("keyAccess")).size()==1,"object");
       write(name+".generated.tdf",object.bytes());
      }
     }
     case "decrypt"->{
      byte[] input=read(name+".tdf");Map<String,Object> manifest;byte[] metadata=new byte[0];boolean hasMetadata;
      try(Reader reader=sdk.loadTDF(input,readerOptions(raw))){
       manifest=reader.manifest();List<Object> kao=(List<Object>)((Map<String,Object>)manifest.get("encryptionInformation")).get("keyAccess");
       hasMetadata=!kao.isEmpty()&&!"".equals(((Map<String,Object>)kao.get(0)).get("encryptedMetadata"));
       if(hasMetadata){reader.init();metadata=reader.unencryptedMetadata();}
      }
      byte[] payload=sdk.decryptBytes(input,readerOptions(raw));
      write(name+".out",payload);write(name+".metadata",metadata);write(name+".manifest",Json.write(manifest));write(name+".presence",Boolean.toString(hasMetadata));
     }
     case "negative"->write(name+".error.json",Json.write(expectFailure("negative",()->sdk.decryptBytes(read(name+".tdf"),readerOptions(raw)))));
     case "repeat"->repeat(sdk,raw);
     default->throw new IllegalArgumentException("unknown mode");
    }
   }
   System.out.println("PASS generated native library "+mode+" "+name);
  }catch(OpenTDFException e){throw new IllegalStateException("consumer failure "+mode+" "+name+" "+Json.write(report(e)),e);}
 }
}
// Test-only JSON utility copied from the Apache2 generic harness; no compiler linkage.
/** Minimal JSON reader and writer; numbers are kept as their source text. */
final class Json {
    private final String s;
    private int i;

    private Json(String s) {
        this.s = s;
    }

    static Object parse(String s) {
        Json j = new Json(s);
        Object v = j.value();
        j.ws();
        if (j.i != s.length()) throw new IllegalArgumentException("trailing data");
        return v;
    }

    private void ws() {
        while (i < s.length() && Character.isWhitespace(s.charAt(i))) i++;
    }

    private Object value() {
        ws();
        if (i >= s.length()) throw new IllegalArgumentException("unexpected end");
        char c = s.charAt(i);
        switch (c) {
            case '{': {
                i++;
                Map<String, Object> m = new LinkedHashMap<>();
                ws();
                if (s.charAt(i) == '}') {
                    i++;
                    return m;
                }
                for (;;) {
                    ws();
                    String k = str();
                    ws();
                    expect(':');
                    m.put(k, value());
                    ws();
                    if (s.charAt(i) == ',') {
                        i++;
                        continue;
                    }
                    expect('}');
                    return m;
                }
            }
            case '[': {
                i++;
                List<Object> l = new ArrayList<>();
                ws();
                if (s.charAt(i) == ']') {
                    i++;
                    return l;
                }
                for (;;) {
                    l.add(value());
                    ws();
                    if (s.charAt(i) == ',') {
                        i++;
                        continue;
                    }
                    expect(']');
                    return l;
                }
            }
            case '"':
                return str();
            case 't':
                i += 4;
                return Boolean.TRUE;
            case 'f':
                i += 5;
                return Boolean.FALSE;
            case 'n':
                i += 4;
                return null;
        }
        int st = i;
        while (i < s.length() && "+-0123456789.eE".indexOf(s.charAt(i)) >= 0) i++;
        if (st == i) throw new IllegalArgumentException("unexpected character " + c);
        return s.substring(st, i);
    }

    private void expect(char c) {
        if (i >= s.length() || s.charAt(i) != c) throw new IllegalArgumentException("expected " + c);
        i++;
    }

    private String str() {
        expect('"');
        StringBuilder b = new StringBuilder();
        for (;;) {
            char c = s.charAt(i++);
            if (c == '"') return b.toString();
            if (c != '\\') {
                b.append(c);
                continue;
            }
            char e = s.charAt(i++);
            switch (e) {
                case 'n' -> b.append('\n');
                case 't' -> b.append('\t');
                case 'r' -> b.append('\r');
                case 'b' -> b.append('\b');
                case 'f' -> b.append('\f');
                case 'u' -> {
                    b.append((char) Integer.parseInt(s.substring(i, i + 4), 16));
                    i += 4;
                }
                default -> b.append(e);
            }
        }
    }

    static String write(Object v) {
        StringBuilder b = new StringBuilder();
        write(b, v);
        return b.toString();
    }

    private static void write(StringBuilder b, Object v) {
        if (v == null) {
            b.append("null");
        } else if (v instanceof String str) {
            b.append('"');
            for (int k = 0; k < str.length(); k++) {
                char c = str.charAt(k);
                switch (c) {
                    case '"' -> b.append("\\\"");
                    case '\\' -> b.append("\\\\");
                    case '\n' -> b.append("\\n");
                    case '\r' -> b.append("\\r");
                    case '\t' -> b.append("\\t");
                    default -> {
                        if (c < 0x20) b.append(String.format("\\u%04x", (int) c));
                        else b.append(c);
                    }
                }
            }
            b.append('"');
        } else if (v instanceof Boolean) {
            b.append(v);
        } else if (v instanceof Map<?, ?> m) {
            b.append('{');
            boolean first = true;
            for (Map.Entry<?, ?> e : m.entrySet()) {
                if (!first) b.append(',');
                first = false;
                write(b, e.getKey());
                b.append(':');
                write(b, e.getValue());
            }
            b.append('}');
        } else if (v instanceof List<?> l) {
            b.append('[');
            for (int k = 0; k < l.size(); k++) {
                if (k > 0) b.append(',');
                write(b, l.get(k));
            }
            b.append(']');
        } else {
            b.append(v);
        }
    }
}
