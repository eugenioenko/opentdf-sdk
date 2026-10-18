package consumer;

import io.opentdf.tdf3.AssertionKey;
import io.opentdf.tdf3.OpenTDFException;
import io.opentdf.tdf3.Reader;
import io.opentdf.tdf3.ReaderOptions;
import io.opentdf.tdf3.SDK;
import io.opentdf.tdf3.TDF3;
import io.opentdf.tdf3.TDFObject;
import io.opentdf.tdf3.TDFOptions;
import io.opentdf.tdf3.protocol.authorization.Entity;
import io.opentdf.tdf3.protocol.policy.Attribute;
import io.opentdf.tdf3.protocol.policy.AttributeRuleTypeEnum;
import io.opentdf.tdf3.protocol.policy.Value;
import io.opentdf.tdf3.protocol.policy.attributes.CreateAttributeRequest;
import io.opentdf.tdf3.protocol.policy.attributes.GetAttributeRequest;
import io.opentdf.tdf3.protocol.policy.attributes.ListAttributesRequest;
import io.opentdf.tdf3.protocol.policy.namespaces.CreateNamespaceRequest;
import io.opentdf.tdf3.protocol.policy.namespaces.ListNamespacesRequest;
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

/** Independent importing consumer of the handle API: one SDK per run, createTDF/decryptBytes, loadTDF readers, 100 round trips, the platform service clients, and the controlled checks of tests/interop/delivery/controlled_platform.py. Host OAuth uses native JCA/HTTP. */
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
 static SDK.Builder builder(Map<String,Object> cfg){return builder(cfg,text(cfg,"PlatformURL"));}
 static SDK.Builder builder(Map<String,Object> cfg,String endpoint){
  SDK.Builder b=SDK.builder(endpoint).insecurePlaintext(bool(cfg,"AllowHTTP"));
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
  if(e instanceof OpenTDFException o){m.put("kind",o.kind());m.put("op",o.op());m.put("httpStatus",o.httpStatus());m.put("connectCode",o.connectCode());m.put("assertionId",o.assertionId());m.put("message",o.getMessage());}
  else m.put("untyped",e.getClass().getName()+": "+e.getMessage());
  return m;
 }
 static Map<String,Object> expectFailure(String label,Callable<?> call){try{call.call();throw new AssertionError(label+" returned successful output");}catch(OpenTDFException e){return report(e);}catch(AssertionError e){throw e;}catch(Exception e){throw new AssertionError(label+" untyped: "+e,e);}}

 @SuppressWarnings("unchecked")static ReaderOptions withKeys(Map<String,Object> cfg,AssertionKey defaultKey,Map<String,AssertionKey> keys){
  List<String> allow=new ArrayList<>();allow.add(text(cfg,"PlatformURL"));if(!text(cfg,"KASURL").isEmpty())allow.add(text(cfg,"KASURL"));
  if(cfg.get("AllowedKAS") instanceof List<?> routes)for(Object r:routes)allow.add(text((Map<String,Object>)r,"URL"));
  ReaderOptions.Builder o=ReaderOptions.builder().kasAllowlist(allow).assertionVerificationKeys(defaultKey,keys);if(!text(cfg,"SessionAlgorithm").isEmpty())o.sessionKeyType(text(cfg,"SessionAlgorithm"));return o.build();
 }
 /** A TDF with an unsigned assertion (HS256 over the payload key) and one signed with a caller HS256 secret, read with the right key by id, with a wrong key and with no key (assertion_failure), and with verification off. */
 @SuppressWarnings("unchecked")static void assertions(SDK sdk,Map<String,Object> cfg)throws Exception{
  String secret="assertion-secret-"+Long.toString(System.nanoTime(),36);
  String firstValue="{\"ocl\":{\"pol\":\"2ccf11cb-6c9a-4e49-9746-a7f0a295945d\"}}";
  List<TDFOptions.AssertionConfig> configs=List.of(
   new TDFOptions.AssertionConfig("unsigned","handling","tdo","encrypted","json+stanag5636","urn:nato:stanag:5636:A:1:elements:json",firstValue),
   new TDFOptions.AssertionConfig("signed","other","payload","unencrypted","json","urn:example:assertion:v1","{\"note\":\"independent consumer\"}",AssertionKey.hs256(secret)));
  byte[] payload="assertion payload".getBytes(StandardCharsets.UTF_8);
  byte[] archive;List<Map<String,Object>> found;
  try(TDFObject object=sdk.createTDF(payload,tdfOptions(cfg).assertions(configs).build())){archive=object.bytes();found=(List<Map<String,Object>>)object.manifest().get("assertions");}
  check(found.size()==2&&"unsigned".equals(found.get(0).get("id"))&&"signed".equals(found.get(1).get("id"))&&"jws".equals(((Map<String,Object>)found.get(1).get("binding")).get("method"))&&!"".equals(((Map<String,Object>)found.get(1).get("binding")).get("signature")),"assertions in manifest");
  check(firstValue.equals(((Map<String,Object>)found.get(0).get("statement")).get("value")),"statement value kept");
  check(Arrays.equals(payload,sdk.decryptBytes(archive,withKeys(cfg,null,Map.of("signed",AssertionKey.hs256(secret))))),"decrypt with the right key");
  Map<String,Object> wrong=expectFailure("wrong key",()->sdk.decryptBytes(archive,withKeys(cfg,null,Map.of("signed",AssertionKey.hs256("not the secret")))));
  check("assertion_failure".equals(wrong.get("kind"))&&"signed".equals(wrong.get("assertionId")),"wrong key: "+Json.write(wrong));
  Map<String,Object> missing=expectFailure("missing key",()->sdk.decryptBytes(archive,readerOptions(cfg)));
  check("assertion_failure".equals(missing.get("kind"))&&"signed".equals(missing.get("assertionId")),"missing key: "+Json.write(missing));
  Map<String,Object> wrongDefault=expectFailure("wrong default key",()->sdk.decryptBytes(archive,withKeys(cfg,AssertionKey.hs256("not the secret"),null)));
  check("assertion_failure".equals(wrongDefault.get("kind"))&&"unsigned".equals(wrongDefault.get("assertionId")),"wrong default key: "+Json.write(wrongDefault));
  List<String> allow=new ArrayList<>();allow.add(text(cfg,"PlatformURL"));if(!text(cfg,"KASURL").isEmpty())allow.add(text(cfg,"KASURL"));
  check(Arrays.equals(payload,sdk.decryptBytes(archive,ReaderOptions.builder().kasAllowlist(allow).disableAssertionVerification(true).build())),"decrypt with verification disabled");
  write("assertions.generated.tdf",archive);
  Map<String,Object> out=new LinkedHashMap<>();List<Object> ids=new ArrayList<>();for(Map<String,Object> a:found){Map<String,Object> row=new LinkedHashMap<>();row.put("id",a.get("id"));row.put("method",((Map<String,Object>)a.get("binding")).get("method"));ids.add(row);}
  out.put("assertions",ids);out.put("wrong_key",wrong);out.put("missing_key",missing);write("assertions.json",Json.write(out));
 }

 /** Phase 7: encryptTo from a file channel into a file (ZIP32 for a seekable input), loadTDF over the channel with asInputStream/seek/readAt/writeTo, then decryptTo and decryptFile. */
 @SuppressWarnings("unchecked")static void streams(SDK sdk,Map<String,Object> cfg)throws Exception{
  byte[] payload=new byte[256*300];for(int i=0;i<payload.length;i++)payload[i]=(byte)i;
  Path plain=run.resolve("stream.plain"),encrypted=run.resolve("stream.generated.tdf"),decrypted=run.resolve("stream.decrypted");
  Files.write(plain,payload);
  try(java.nio.channels.FileChannel in=java.nio.channels.FileChannel.open(plain);java.io.OutputStream out=Files.newOutputStream(encrypted);TDFObject object=sdk.encryptTo(out,in,tdfOptions(cfg).segmentSize(16384).build())){
   Map<String,Object> integrity=(Map<String,Object>)((Map<String,Object>)object.manifest().get("encryptionInformation")).get("integrityInformation");
   check(object.bytes().length==0&&Long.valueOf(16384).equals(((Map<String,Object>)((List<Object>)integrity.get("segments")).get(0)).get("segmentSize")),"streamed object");
  }
  check(SDK.isValidTdf(Files.readAllBytes(encrypted)),"streamed archive is a TDF");
  try(java.nio.channels.FileChannel in=java.nio.channels.FileChannel.open(encrypted);Reader reader=sdk.loadTDF(in,readerOptions(cfg))){
   reader.init();java.io.InputStream stream=reader.asInputStream();
   check(Arrays.equals(stream.readNBytes(5),Arrays.copyOf(payload,5))&&reader.seek(-10,2)==payload.length-10&&Arrays.equals(stream.readAllBytes(),Arrays.copyOfRange(payload,payload.length-10,payload.length)),"read and seek");
   check(Arrays.equals(reader.readAt(16380,8),Arrays.copyOfRange(payload,16380,16388))&&reader.seek(0,0)==0,"readAt");
   java.io.ByteArrayOutputStream out=new java.io.ByteArrayOutputStream();
   check(reader.writeTo(out)==payload.length&&Arrays.equals(out.toByteArray(),payload),"writeTo");
  }
  try(java.nio.channels.FileChannel in=java.nio.channels.FileChannel.open(encrypted)){java.io.ByteArrayOutputStream out=new java.io.ByteArrayOutputStream();sdk.decryptTo(out,in,readerOptions(cfg));check(Arrays.equals(out.toByteArray(),payload),"decryptTo");}
  sdk.decryptFile(encrypted.toString(),decrypted.toString(),readerOptions(cfg));
  check(Arrays.equals(Files.readAllBytes(decrypted),payload),"decryptFile");
  Map<String,Object> notTdf=expectFailure("plain file",()->{sdk.decryptFile(plain.toString(),decrypted.toString(),readerOptions(cfg));return null;});
  check("not_decryptable".equals(notTdf.get("kind"))&&Arrays.equals(Files.readAllBytes(decrypted),payload),"decryptFile keeps the output: "+Json.write(notTdf));
 }

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
  streams(sdk,cfg);
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

 /** xtest's test_self.py through the service clients: list namespaces, create a namespace and an attribute, read the attribute back by id, through the namespace's list and through the raw JSON call, and check a missing attribute's error. */
 @SuppressWarnings("unchecked")static void services(SDK sdk,Map<String,Object> cfg)throws Exception{
  String ns="gcjava"+Long.toString(System.nanoTime(),36)+".com";
  var before=sdk.namespaces().listNamespaces(ListNamespacesRequest.getDefaultInstance());
  var namespace=sdk.namespaces().createNamespace(CreateNamespaceRequest.newBuilder().setName(ns).build()).getNamespace();
  check(!namespace.getId().isEmpty()&&namespace.getName().equals(ns),"created namespace");
  var after=sdk.namespaces().listNamespacesAsync(ListNamespacesRequest.getDefaultInstance()).get(60,TimeUnit.SECONDS);
  check(after.getNamespacesList().stream().anyMatch(n->n.getId().equals(namespace.getId()))&&after.getNamespacesCount()==before.getNamespacesCount()+1,"namespace listed");
  var created=sdk.attributes().createAttribute(CreateAttributeRequest.newBuilder().setNamespaceId(namespace.getId()).setName("free").setRule(AttributeRuleTypeEnum.ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF).addAllValues(List.of("1","2","3")).build());
  var attribute=sdk.attributes().getAttribute(GetAttributeRequest.newBuilder().setAttributeId(created.getAttribute().getId()).build()).getAttribute();
  List<String> values=attribute.getValuesList().stream().map(v->v.getValue()).toList();
  check(attribute.getFqn().equals("https://"+ns+"/attr/free")&&attribute.getRule()==AttributeRuleTypeEnum.ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF&&values.equals(List.of("1","2","3"))&&attribute.getNamespace().getName().equals(ns),"attribute read back");
  var listed=sdk.attributes().listAttributes(ListAttributesRequest.newBuilder().setNamespace(ns).build());
  check(listed.getAttributesCount()==1&&listed.getAttributes(0).getId().equals(attribute.getId()),"attribute listed");
  Map<String,Object> raw=(Map<String,Object>)Json.parse(sdk.call("policy.attributes.AttributesService","GetAttribute",Json.write(Map.of("attributeId",attribute.getId()))));
  check(attribute.getFqn().equals(((Map<String,Object>)raw.get("attribute")).get("fqn")),"raw JSON call");
  Map<String,Object> missing=expectFailure("missing attribute",()->sdk.attributes().getAttribute(GetAttributeRequest.newBuilder().setAttributeId("00000000-0000-0000-0000-000000000000").build()));
  check("not_found".equals(missing.get("connectCode"))&&Long.valueOf(404).equals(missing.get("httpStatus")),"missing attribute error: "+missing);
  Map<String,Object> report=new LinkedHashMap<>();
  report.put("namespace",ns);report.put("namespaces_before",before.getNamespacesCount());report.put("namespaces_after",after.getNamespacesCount());
  report.put("attribute_fqn",attribute.getFqn());report.put("values",values);report.put("not_found",missing);
  report.put("helpers",helpers(sdk,cfg,ns,attribute.getFqn()));
  report.put("autoconfigure",autoconfigure(sdk,cfg,attribute));
  write("services.json",Json.write(report));
 }

 /** Encrypts under a value of the attribute created, looked up (attributes) and offline (attributeValues): the attribute has no grants, so the plan is the base key or the default KAS. */
 @SuppressWarnings("unchecked")static Map<String,Object> autoconfigure(SDK sdk,Map<String,Object> cfg,Attribute attribute)throws Exception{
  String value=attribute.getFqn()+"/value/1";
  Value offline=Value.newBuilder().setFqn(value).setValue("1").setAttribute(Attribute.newBuilder().setFqn(attribute.getFqn()).setRule(attribute.getRule()).setNamespace(attribute.getNamespace())).build();
  Map<String,Object> out=new LinkedHashMap<>();
  out.put("key",SDK.newAttributeValueFQN(value).key());
  Map<String,TDFOptions.Builder> paths=new LinkedHashMap<>();
  paths.put("looked_up",TDFOptions.builder().kasInformation(kasInformation(cfg)).attributes(value));
  paths.put("offline",TDFOptions.builder().kasInformation(kasInformation(cfg)).attributeValues(offline));
  for(var path:paths.entrySet()){
   try(TDFObject object=sdk.createTDF(path.getKey().getBytes(StandardCharsets.UTF_8),path.getValue().build())){
    List<Map<String,Object>> kaos=(List<Map<String,Object>>)((Map<String,Object>)object.manifest().get("encryptionInformation")).get("keyAccess");
    Map<String,Object> kao=kaos.get(0);
    out.put(path.getKey(),Map.of("kaos",kaos.size(),"kas",kao.get("url"),"sid",kao.getOrDefault("sid","")));
   }
  }
  return out;
 }

 /** The reference's attribute helpers on the attribute services created, SDK.auth, and an SDK whose platform endpoint is unreachable but whose core connection is the platform. */
 static Map<String,Object> helpers(SDK sdk,Map<String,Object> cfg,String ns,String fqn)throws Exception{
  Map<String,Object> h=new LinkedHashMap<>();
  h.put("attribute_exists",sdk.attributeExists(fqn));h.put("attribute_missing",sdk.attributeExists(fqn+"-missing"));
  h.put("value_exists",sdk.attributeValueExists(fqn+"/value/1"));h.put("value_missing",sdk.attributeValueExistsAsync(fqn+"/value/9").get(60,TimeUnit.SECONDS));
  h.put("listed",sdk.listAttributes(ns).stream().map(a->a.getFqn()).toList());
  sdk.validateAttributes(List.of(fqn+"/value/1",fqn+"/value/2"));h.put("validate_ok",true);
  h.put("validate_missing",expectFailure("validate missing",()->{sdk.validateAttributes(List.of(fqn+"/value/1",fqn+"/value/9"));return null;}));
  h.put("entity_attributes",sdk.getEntityAttributes(Entity.newBuilder().setId("e1").setClientId("opentdf-sdk").build()));
  TDF3.AccessToken token=sdk.auth().accessToken();
  h.put("access_token",Map.of("present",!token.value().isEmpty(),"scheme",token.scheme()));
  try(SDK override=builder(cfg,"http://127.0.0.1:9").coreEndpoint(text(cfg,"PlatformURL")).build()){
   Map<String,Object> o=new LinkedHashMap<>();o.put("endpoint",override.conn().endpoint());o.put("attribute_exists",override.attributeExists(fqn));h.put("override",o);
  }
  return h;
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
      boolean split=name.equals("split")||name.endsWith("-split");
      if(split)options.kasInformation(kasInformation(raw),kasInformation(raw));
      try(TDFObject object=sdk.createTDF(read(name+".input"),options.build())){
       check(object.size()==object.bytes().length&&((List<Object>)((Map<String,Object>)object.manifest().get("encryptionInformation")).get("keyAccess")).size()==(split?2:1),"object");
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
     case "assertions"->assertions(sdk,raw);
     case "repeat"->repeat(sdk,raw);
     case "services"->services(sdk,raw);
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
