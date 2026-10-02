package consumer;

import io.opentdf.tdf3.TDF3;
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

/** Independent importing consumer. Host OAuth uses native JCA/HTTP, with host-only credentials. */
public final class Consumer {
 static Path run;static Map<String,Object> raw;static TDF3.Config config;
 static String text(Map<String,Object> o,String k){return o.get(k) instanceof String s?s:"";}
 static boolean bool(Map<String,Object> o,String k){return Boolean.TRUE.equals(o.get(k));}
 static void check(boolean ok,String name){if(!ok)throw new AssertionError(name);}
 static byte[] read(String name)throws Exception{return Files.readAllBytes(run.resolve(name));}
 static void write(String name,byte[] bytes)throws Exception{Path p=run.resolve(name);Files.write(p,bytes);Files.setPosixFilePermissions(p,java.nio.file.attribute.PosixFilePermissions.fromString("rw-------"));}
 static void write(String name,String text)throws Exception{write(name,text.getBytes(StandardCharsets.UTF_8));}
 static <T>T await(TDF3.Operation<T> op)throws Exception{
  try{return op.completion().toCompletableFuture().get(90,TimeUnit.SECONDS);}
  catch(ExecutionException e){if(e.getCause() instanceof Exception x)throw x;throw e;}
 }
 @SuppressWarnings("unchecked")static TDF3.Config cfg(Map<String,Object> r){
  TDF3.Config c=new TDF3.Config();c.PlatformURL=text(r,"PlatformURL");c.KASURL=text(r,"KASURL");c.IssuerURL=text(r,"IssuerURL");c.TokenURL=text(r,"TokenURL");c.ClientID=text(r,"ClientID");c.ClientSecret=text(r,"ClientSecret");c.TokenProviderName=text(r,"TokenProviderName");
  c.KASPublicKeyPEM=text(r,"KASPublicKeyPEM");c.KID=text(r,"KID");c.KASAlgorithm=text(r,"KASAlgorithm");c.SessionAlgorithm=text(r,"SessionAlgorithm");c.AuthPrivateKeyPEM=text(r,"AuthPrivateKeyPEM");c.AuthAlgorithm=text(r,"AuthAlgorithm");c.DPoP=bool(r,"DPoP");c.AllowHTTP=bool(r,"AllowHTTP");if(r.containsKey("TimeoutMillis"))c.TimeoutMillis=Long.parseLong(text(r,"TimeoutMillis"));
  if(r.get("AllowedKAS") instanceof List<?> a){c.AllowedKAS=new TDF3.KASRoute[a.size()];for(int i=0;i<a.size();i++){Map<String,Object> v=(Map<String,Object>)a.get(i);TDF3.KASRoute route=new TDF3.KASRoute();route.URL=text(v,"URL");route.APIBaseURL=text(v,"APIBaseURL");c.AllowedKAS[i]=route;}}
  return c;
 }
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
 @SuppressWarnings("unchecked")static TDF3.AccessToken token(String name,TDF3.ProviderRequest request)throws Exception{
  long now=Instant.now().getEpochSecond();if(name.equals("invalid-token"))return new TDF3.AccessToken("invalid-token","Bearer",now+300,"");if(name.equals("expired-token"))return new TDF3.AccessToken("expired-token","Bearer",now-1,"");if(name.equals("provider-reject"))throw new IllegalArgumentException("provider rejected");
  String endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token",nonce="",thumb="";PrivateKey key=null;Map<String,Object> jwk=null;
  if(config.DPoP){if(name.equals("mismatched-provider")){KeyPairGenerator generator=KeyPairGenerator.getInstance("EC");generator.initialize(new ECGenParameterSpec("secp256r1"));key=generator.generateKeyPair().getPrivate();}else key=privateKey(config.AuthPrivateKeyPEM);jwk=jwk(key);thumb=b64(MessageDigest.getInstance("SHA-256").digest(Json.write(jwk).getBytes(StandardCharsets.UTF_8)));}
  Thread thread=Thread.currentThread();
  try(HttpClient client=HttpClient.newBuilder().followRedirects(HttpClient.Redirect.NEVER).connectTimeout(Duration.ofSeconds(10)).build()){
   request.onStop(()->{thread.interrupt();client.shutdownNow();});
   for(int attempt=0;attempt<3;attempt++){
    HttpRequest.Builder builder=HttpRequest.newBuilder(URI.create(endpoint)).timeout(Duration.ofSeconds(15)).header("Content-Type","application/x-www-form-urlencoded").POST(HttpRequest.BodyPublishers.ofString("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret"));if(config.DPoP)builder.header("DPoP",proof(key,jwk,endpoint,nonce));HttpResponse<byte[]> response=client.send(builder.build(),HttpResponse.BodyHandlers.ofByteArray());
    if(response.body().length>128<<10)throw new IllegalArgumentException("provider body limit");String next=response.headers().firstValue("DPoP-Nonce").orElse("");if((response.statusCode()==400||response.statusCode()==401)&&!next.isEmpty()&&!next.equals(nonce)){nonce=next;continue;}if(response.statusCode()!=200)throw new IllegalArgumentException("provider status");Map<String,Object> value=(Map<String,Object>)Json.parse(new String(response.body(),StandardCharsets.UTF_8));return new TDF3.AccessToken(text(value,"access_token"),config.DPoP?"DPoP":"Bearer",Instant.now().getEpochSecond()+Long.parseLong(text(value,"expires_in")),thumb);
   }
  }
  throw new IllegalArgumentException("provider nonce retries");
 }
 static TDF3.CallOptions call(String name){TDF3.CallOptions c=new TDF3.CallOptions();if(!config.TokenProviderName.isEmpty())c.tokenProvider=request->{request.retain();Thread.ofVirtual().start(()->{try{TDF3.AccessToken t=token(name,request);request.resolve(t);}catch(Exception e){request.reject("provider: rejected");}});};return c;}
 static TDF3.EncryptOptions opts(String name){TDF3.EncryptOptions o=new TDF3.EncryptOptions();o.Attributes=new String[]{"https://example.com/attr/attr1/value/"+(name.equals("denied")||name.endsWith("-denied")?"value2":"value1")};o.SegmentSize=16384;o.HasSegmentSize=true;if(name.equals("metadata")||name.endsWith("-metadata")){o.Metadata="{\"source\":\"independent metadata\",\"count\":7}".getBytes(StandardCharsets.UTF_8);o.IncludeMetadata=true;}if(name.equals("empty-metadata")||name.endsWith("-empty-metadata")){o.Metadata=new byte[0];o.IncludeMetadata=true;}if(name.equals("hs256")||name.endsWith("-hs256"))o.SegmentHashAlgorithm="HS256";return o;}
 static void error(String name,TDF3.TDFError e)throws Exception{Map<String,Object> value=new LinkedHashMap<>();value.put("kind",e.kind);value.put("code",e.code);value.put("operation",e.operation);value.put("httpStatus",e.httpStatus);value.put("causeCategory",e.causeCategory);write(name+".error.json",Json.write(value));}
 static void repeat()throws Exception{
  byte[] payload={0,(byte)255,(byte)128,1},metadata={0,(byte)255};TDF3.EncryptOptions options=opts("metadata");options.Metadata=metadata;options.MimeType="application/日本語";byte[] archive=await(TDF3.encrypt(config,payload,options,call("ownership")));TDF3.Decrypted a=await(TDF3.decrypt(config,archive,call("ownership"))),b=await(TDF3.decrypt(config,archive,call("ownership")));check(Arrays.equals(a.Payload(),payload)&&Arrays.equals(a.Metadata(),metadata),"durable bytes");check(a.ManifestJSON().contains("日本語"),"UTF8 manifest");byte[] own=b.Payload();own[0]=44;metadata[0]=44;check(a.Payload()[0]==0&&a.Metadata()[0]==0,"owned output");
  CountDownLatch started=new CountDownLatch(1);TDF3.CallOptions held=new TDF3.CallOptions();held.tokenProvider=request->{request.retain();request.onStop(()->request.reject("provider: stopped"));started.countDown();};TDF3.Config fresh=cfg(raw);fresh.ClientID="";fresh.ClientSecret="";TDF3.Operation<TDF3.Decrypted> active=TDF3.decrypt(fresh,archive,held);check(started.await(20,TimeUnit.SECONDS),"provider started");TDF3.Operation<TDF3.Decrypted> queued=TDF3.decrypt(config,archive,call("ownership"));check(queued.cancel(),"queued cancel");try{await(queued);throw new AssertionError("queued success");}catch(TDF3.TDFError e){check(e.kind.equals("canceled"),"queued category");}check(active.cancel(),"active cancel");try{await(active);throw new AssertionError("active success");}catch(TDF3.TDFError e){check(e.kind.equals("canceled"),"active category");}check(Arrays.equals(await(TDF3.decrypt(config,archive,call("ownership"))).Payload(),payload),"nextcall recovery");
 }
 static void controlled(String name)throws Exception{
  TDF3.CallOptions call=new TDF3.CallOptions();call.tokenProvider=request->request.resolve(new TDF3.AccessToken("negative-fixture-token","Bearer",Instant.now().getEpochSecond()+300,""));
  var operation=TDF3.decrypt(config,read("archive.tdf"),call);Thread cancel=null;
  if(name.equals("active-cancel"))cancel=Thread.ofVirtual().start(()->{try{long end=System.nanoTime()+TimeUnit.SECONDS.toNanos(15);while(!Files.exists(run.resolve("entered"))){if(System.nanoTime()>end)throw new AssertionError("HTTP not acquired");Thread.sleep(2);}operation.cancel();}catch(Exception e){throw new IllegalStateException(e);}});
  TDF3.TDFError error;try{await(operation);throw new AssertionError("controlled endpoint returned plaintext");}catch(TDF3.TDFError e){error=e;}
  String expected=switch(name){case "http401"->"unauthenticated";case "http403"->"denied";case "redirect"->"http_status";case "content-type"->"invalid_content_type";case "malformed"->"invalid_json";case "active-cancel"->"canceled";case "bom"->"invalid_destination";default->"transport";};
  check(error.code.equals(expected),"controlled category "+name+" "+error.code);if(name.equals("http401")||name.equals("http403"))check(error.serverCode.equals("fixture-rejected")&&error.serverMessage.equals("診断 café"),"UTF8 server error");
  if(cancel!=null)cancel.join();error(name,error);
  try{await(TDF3.decrypt(new TDF3.Config(),new byte[]{0},null));throw new AssertionError("next invalid config success");}catch(TDF3.TDFError e){check(e.kind.equals("source"),"next-call recovery");}
 }
 @SuppressWarnings("unchecked")public static void main(String[] args)throws Exception{
  if(args.length!=4)throw new IllegalArgumentException("consumer sdk run mode case");run=Path.of(args[1]);String mode=args[2],name=args[3];raw=(Map<String,Object>)Json.parse(Files.readString(run.resolve("config.json")));config=cfg(raw);
  try{
   switch(mode){
    case "encrypt"->write(name+".generated.tdf",await(TDF3.encrypt(config,read(name+".input"),opts(name),call(name))));
    case "decrypt"->{TDF3.Decrypted result=await(TDF3.decrypt(config,read(name+".tdf"),call(name)));write(name+".out",result.Payload());write(name+".metadata",result.Metadata());write(name+".manifest",result.ManifestJSON());write(name+".presence",Boolean.toString(result.HasMetadata()));}
    case "negative"->{try{await(TDF3.decrypt(config,read(name+".tdf"),call(name)));throw new AssertionError("negative returned plaintext");}catch(TDF3.TDFError e){error(name,e);}}
    case "repeat"->repeat();
    case "controlled"->controlled(name);
    default->throw new IllegalArgumentException("unknown mode");
   }
  }catch(TDF3.TDFError e){throw new IllegalStateException("consumer failure "+mode+" "+name+" kind="+e.kind+" code="+e.code+" operation="+e.operation+" cause="+e.causeCategory);}
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
