"""Installed SDK buffer byte-count and pre-source submission contract checks."""
import array,importlib,threading
import opentdf_tdf3 as sdk
from opentdf_tdf3._generated import rt
library=importlib.import_module('opentdf_tdf3._generated.rt.types.library')
checks=0

def check(value,label):
 global checks
 assert value,label;checks+=1

def rejection(value):
 try:library.library_bytes(value);raise AssertionError('oversize accepted')
 except rt.LibraryFailure as e:check(e.kind=='invalid_argument','helper category')

limit=library.LIMIT
library.LIMIT=4
try:
 for value in (b'abcd',bytearray(b'abcd'),memoryview(b'abcd'),memoryview(b'abcd').cast('I'),memoryview(b'abcd').cast('B',shape=[2,2]),memoryview(b'axbxcxdx')[::2]):
  copied=library.library_bytes(value);check(bytes(copied)==b'abcd','bounded raw-byte representation')
 for value in (b'abcde',bytearray(b'abcde'),memoryview(b'abcde'),memoryview(bytes(8)).cast('I'),memoryview(bytes(8)).cast('B',shape=[2,4])):
  rejection(value)
 released=memoryview(b'a');released.release();rejection(released)
 mutable=bytearray(b'abcd');view=memoryview(mutable).cast('B',shape=[2,2]);copied=library.library_bytes(view);mutable[:]=b'wxyz';check(bytes(copied)==b'abcd','helper owns multidimensional bytes')
 entered=[]
 original_encrypt=sdk._generated.Encrypt;original_decrypt=sdk._generated.Decrypt
 def spy_encrypt(config,payload,options,call):
  entered.append(('encrypt',payload,options))
  op=rt.LibraryOperation();op._future.set_result(payload);return op
 def spy_decrypt(config,payload,call):
  entered.append(('decrypt',payload))
  op=rt.LibraryOperation();op._future.set_result({'Payload':payload,'Metadata':b'','HasMetadata':False,'ManifestJSON':b'{}'});return op
 sdk._generated.Encrypt=spy_encrypt;sdk._generated.Decrypt=spy_decrypt
 try:
  for value in (memoryview(bytes(8)).cast('I'),memoryview(bytes(8)).cast('B',shape=[2,4]),released):
   for operation in (sdk.encrypt(sdk.Config(),value),sdk.decrypt(sdk.Config(),value),sdk.encrypt(sdk.Config(),b'',sdk.EncryptOptions(metadata=value,include_metadata=True))):
    try:operation.result(5);raise AssertionError('source entered for rejected input')
    except sdk.TDFError as e:check(e.kind=='invalid_argument','SDK declared validation')
  check(not entered,'all excessive/released views rejected before source submission')
  mutable=bytearray(b'abcd');view=memoryview(mutable).cast('B',shape=[2,2]);options=sdk.EncryptOptions(metadata=view,include_metadata=True)
  operation=sdk.encrypt(sdk.Config(),view,options);mutable[:]=b'wxyz'
  check(operation.result()==b'abcd' and entered[-1][2]['Metadata']==b'abcd','SDK owns payload and metadata before submit')
  mutable=bytearray(b'abcd');view=memoryview(mutable).cast('I');operation=sdk.decrypt(sdk.Config(),view);mutable[:]=b'wxyz'
  check(operation.result().payload==b'abcd','SDK owns typed view before submit')
 finally:sdk._generated.Encrypt=original_encrypt;sdk._generated.Decrypt=original_decrypt
finally:library.LIMIT=limit
# Actual generated SDK remains callable after validation rejection; imports the
# supported omitted-Q auth key before the shared invalid destination check.
import base64
omitted_der=bytes.fromhex('3041020100301306072a8648ce3d020106082a8648ce3d030107042730250201010420'+'00'*31+'01')
omitted_pem=(b'-----BEGIN PRIVATE KEY-----\n'+base64.encodebytes(omitted_der)+b'-----END PRIVATE KEY-----\n').decode()
try:sdk.encrypt_sync(sdk.Config(auth_private_key_pem=omitted_pem,auth_algorithm='ES256'),b'');raise AssertionError('invalid configuration success')
except sdk.TDFError as e:check(e.code=='invalid_destination' and e.operation!='import_auth_key','actual SDK omitted-Q auth import before declared config error')
check(not library._queue,'validation/recovery queue empty')
print('PASS installed SDK buffer/ownership/pre-source checks',checks)
