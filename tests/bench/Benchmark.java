package consumer;
import io.opentdf.tdf3.TDF3;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
/** In-process public Java API benchmark against the installed JAR. */
public final class Benchmark {
    @SuppressWarnings("unchecked")public static void main(String[] args)throws Exception{
        Path run=Path.of(args[0]);
        String op=args[1],size=args[2];
        int n=Integer.parseInt(args[3]);
        Map<String,Object> raw=(Map<String,Object>)Json.parse(Files.readString(run.resolve("private.json")));
        byte[] input=Files.readAllBytes(run.resolve(size+".input")),archive=op.equals("decrypt")?Files.readAllBytes(run.resolve(size+".reference.tdf")):null;
        List<Double> samples=new ArrayList<>();
        for(int i=-1;i<n;i++){
            long start=System.nanoTime();
            TDF3.Config c=Consumer.cfg((Map<String,Object>)raw.get("Config"));
            TDF3.CallOptions call=new TDF3.CallOptions();
            call.tokenProvider=request->request.resolve(new TDF3.AccessToken((String)raw.get("Token"),"Bearer",Long.parseLong(raw.get("Expires").toString()),""));
            byte[] output;
            if(op.equals("encrypt")){
                TDF3.EncryptOptions opts=new TDF3.EncryptOptions();
                opts.Attributes=new String[]{
                    "https://example.com/attr/attr1/value/value1"
                }
                ;
                opts.SegmentSize=2<<20;
                opts.HasSegmentSize=true;
                opts.SegmentHashAlgorithm="GMAC";
                output=TDF3.encrypt(c,input,opts,call).completion().toCompletableFuture().get(1800,TimeUnit.SECONDS);
            }
            else output=TDF3.decrypt(c,archive,call).completion().toCompletableFuture().get(1800,TimeUnit.SECONDS).Payload();
            double ms=(System.nanoTime()-start)/1e6;
            if(op.equals("decrypt")){
                if(!Arrays.equals(input,output))throw new AssertionError("plaintext mismatch");
            }
            else Files.write(run.resolve("java-"+size+"-"+i+".tdf"),output);
            if(i>=0)samples.add(ms);
        }
        System.out.println("{\"samples_ms\":"+samples+",\"correct\":true}");
    }
}
