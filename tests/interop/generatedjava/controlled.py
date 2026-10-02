#!/usr/bin/env python3
"""Actual importing SDK negatives; every endpoint rejects, no positive mock KAS."""
import http.server,threading,subprocess,json,ssl,os,hashlib,time
from pathlib import Path
SDK=Path(__file__).resolve().parents[3]
BASE=Path(os.environ.get('TDF_JAVA_CONTROLLED_OUT',str(SDK/'.local/java-tdf-library/controlled')))
BASE.mkdir(parents=True,exist_ok=True)
archive=SDK/'.local/java-tdf-library/basic-result-fix/binary.generated.tdf'
key,cert=BASE/'server.key',BASE/'server.crt'
subprocess.run(['openssl','req','-x509','-newkey','rsa:2048','-nodes','-keyout',str(key),'-out',str(cert),'-days','1','-subj','/CN=localhost'],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
key.chmod(0o600)
rows=[]
for mode in os.environ.get('TDF_JAVA_CONTROLLED_CASES','').split(',') if os.environ.get('TDF_JAVA_CONTROLLED_CASES') else ['http401','http403','redirect','content-type','malformed','oversize','truncated','tls','deadline','active-cancel','bom']:
 run=BASE/mode;run.mkdir(exist_ok=False);(run/'archive.tdf').write_bytes(archive.read_bytes());contacts=[];sinks=[];handshakes=[];released=threading.Event()
 class Sink(http.server.BaseHTTPRequestHandler):
  def log_message(self,*args):pass
  def do_GET(self):sinks.append('GET');self.send_response(403);self.end_headers()
  def do_POST(self):sinks.append('POST');self.send_response(403);self.end_headers()
 sink=http.server.ThreadingHTTPServer(('127.0.0.1',0),Sink);sinkThread=threading.Thread(target=sink.serve_forever,daemon=True);sinkThread.start()
 class Handler(http.server.BaseHTTPRequestHandler):
  def log_message(self,*args):pass
  def do_POST(self):
   contacts.append({'method':'POST','path':self.path});self.rfile.read(int(self.headers.get('Content-Length','0')))
   status=401 if mode=='http401' else 403 if mode=='http403' else 302 if mode=='redirect' else 200
   body=json.dumps({'code':'fixture-rejected','message':'診断 café'},ensure_ascii=False).encode() if mode in ['http401','http403'] else b'{bad' if mode=='malformed' else b'x'*((1<<20)+2) if mode=='oversize' else b'{}'
   self.send_response(status);self.send_header('Content-Type','text/html' if mode=='content-type' else 'application/json');self.send_header('Content-Length','200' if mode in ['deadline','active-cancel','truncated'] else str(len(body)))
   if mode=='redirect':self.send_header('Location','http://127.0.0.1:'+str(sink.server_port)+'/leak')
   self.end_headers()
   try:
    if mode in ['deadline','active-cancel']:
     self.wfile.write(b'{');self.wfile.flush();(run/'entered').write_text('acquired\n');self.connection.settimeout(15);self.rfile.read(1);released.set()
    else:self.wfile.write(b'{' if mode=='truncated' else body);self.wfile.flush()
   except (BrokenPipeError,ConnectionResetError):released.set()
   finally:self.close_connection=True
 class TLSServer(http.server.ThreadingHTTPServer):
  def get_request(self):
   raw,addr=self.socket.accept();handshakes.append(1)
   try:return tls.wrap_socket(raw,server_side=True),addr
   except ssl.SSLError:raw.close();raise
 server=TLSServer(('127.0.0.1',0),Handler) if mode=='tls' else http.server.ThreadingHTTPServer(('127.0.0.1',0),Handler)
 if mode=='tls':tls=ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER);tls.load_cert_chain(str(cert),str(key))
 serverThread=threading.Thread(target=server.serve_forever,daemon=True);serverThread.start()
 endpoint=('https' if mode=='tls' else 'http')+'://127.0.0.1:'+str(server.server_port)
 cfg={'PlatformURL':'http://localhost:8080','KASURL':'http://localhost:8080/kas','AllowHTTP':True,'TimeoutMillis':1000 if mode=='deadline' else 10000,'AllowedKAS':[{'URL':'http://localhost:8080/kas','APIBaseURL':('\ufeff' if mode=='bom' else '')+endpoint}]}
 (run/'config.json').write_text(json.dumps(cfg));start=time.monotonic()
 try:
  with (run/'consumer.log').open('wb') as log:result=subprocess.run([str(SDK/'tests/interop/generatedjava/consumer.sh'),str(SDK),str(run),'controlled',mode],stdout=log,stderr=subprocess.STDOUT,timeout=30)
  (run/'consumer.status').write_text(str(result.returncode)+'\n')
  if result.returncode:raise RuntimeError('controlled case failed '+mode+'; see '+str(run/'consumer.log'))
  assert not sinks
  if mode in ['tls','bom']:assert not contacts
  else:assert len(contacts)==1
  if mode=='tls':assert handshakes
  if mode in ['deadline','active-cancel']:assert released.wait(3),'connection still held'
  row={'case':mode,'error':json.loads((run/(mode+'.error.json')).read_text()),'contacts':contacts,'redirectContacts':sinks,'tlsHandshakes':len(handshakes),'nativeReleaseObserved':released.is_set(),'elapsedSeconds':time.monotonic()-start,'status':0,'zeroOutput':True}
  rows.append(row);print('PASS generated Java controlled',mode,flush=True)
 finally:server.shutdown();server.server_close();serverThread.join();sink.shutdown();sink.server_close();sinkThread.join()
(BASE/'results.json').write_text(json.dumps({'scope':'actual SDK controlled negatives, no positive mock KAS','cases':rows},indent=2)+'\n')
