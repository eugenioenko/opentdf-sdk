// Independent pinned stock SDK writer; no shared SDK protocol/crypto is reused.
import { createRequire } from 'node:module';
import { readFile, writeFile } from 'node:fs/promises';
const [sdk, input, output, algorithm, metadata, attribute] = process.argv.slice(2);
const require = createRequire(import.meta.url);
const { OpenTDF, AuthProviders } = require(sdk + '/.local/web-cli/node_modules/@opentdf/sdk');
const authProvider = await AuthProviders.clientSecretAuthProvider({
  clientId: 'opentdf-sdk', clientSecret: 'secret',
  oidcOrigin: 'http://localhost:8888/auth/realms/opentdf', exchange: 'client',
});
const client = new OpenTDF({authProvider, disableDPoP: true, platformUrl:'http://localhost:8080', policyEndpoint:'http://localhost:8080',defaultCreateOptions:{defaultKASEndpoint:'http://localhost:8080/kas'}});
try {
 const plaintext = await readFile(input);
 // The pinned high-level OpenTDF facade drops metadata; its stock lower-level
 // client exposes the option and independently implements all wire/crypto work.
 const stream = await client.tdf3Client.encrypt({source:new Blob([plaintext]).stream(),scope:{attributes:attribute?[attribute]:[]},defaultKASEndpoint:'http://localhost:8080/kas',autoconfigure:false,windowSize:16384,wrappingKeyAlgorithm:'rsa:2048',segmentIntegrityAlgorithm:algorithm,metadata:metadata?JSON.parse(metadata):undefined});
 await writeFile(output,Buffer.from(await new Response(stream.stream).arrayBuffer()));
} finally { client.close(); }
