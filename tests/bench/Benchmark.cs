using System;
using System.IO;
using System.Text.Json;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;
using OpenTDF;
static class Benchmark{
    static void Main(string[] args){
        string run=args[0],op=args[1],size=args[2];
        int n=int.Parse(args[3]);
        var raw=JsonDocument.Parse(File.ReadAllText(Path.Combine(run,"private.json"))).RootElement;
        byte[] input=File.ReadAllBytes(Path.Combine(run,size+".input")),archive=op=="decrypt"?File.ReadAllBytes(Path.Combine(run,size+".reference.tdf")):null;
        var samples=new List<double>();
        for(int i=-1;i<n;i++){
            long start=Stopwatch.GetTimestamp();
            var c=JsonSerializer.Deserialize<TDF3.Config>(raw.GetProperty("Config").GetRawText(),new JsonSerializerOptions{
                IncludeFields=true
            }
            );
            var call=new TDF3.CallOptions{
                Provider=request=>request.Resolve(new TDF3.AccessToken(raw.GetProperty("Token").GetString(),"Bearer",raw.GetProperty("Expires").GetInt64()))
            }
            ;
            byte[] output=op=="encrypt"?TDF3.Encrypt(c,input,new TDF3.EncryptOptions{
                Attributes=new[]{
                    "https://example.com/attr/attr1/value/value1"
                }
                ,SegmentSize=2<<20,HasSegmentSize=true,SegmentHashAlgorithm="GMAC"
            }
            ,call).Completion.GetAwaiter().GetResult():TDF3.Decrypt(c,archive,call).Completion.GetAwaiter().GetResult().Payload;
            double ms=Stopwatch.GetElapsedTime(start).TotalMilliseconds;
            if(op=="decrypt"){
                if(!output.SequenceEqual(input))throw new Exception("plaintext mismatch");
            }
            else File.WriteAllBytes(Path.Combine(run,$"csharp-{size}-{i}.tdf"),output);
            if(i>=0)samples.Add(ms);
        }
        Console.WriteLine(JsonSerializer.Serialize(new{
            samples_ms=samples,correct=true
        }
        ));
    }
}
