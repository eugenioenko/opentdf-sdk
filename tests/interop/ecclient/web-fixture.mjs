// Independent pinned stock SDK writer; no shared SDK protocol/crypto is reused.
import { createRequire } from 'node:module';
import { readFile, writeFile } from 'node:fs/promises';
const [sdk, input, output, algorithm, metadata, attribute, wrapping, kid, publicKeyPath] = process.argv.slice(2);
const require = createRequire(import.meta.url);
const { OpenTDF, AuthProviders } = require(sdk + '/.local/web-cli/node_modules/@opentdf/sdk');
const authProvider = await AuthProviders.clientSecretAuthProvider({
  clientId: 'opentdf-sdk', clientSecret: 'secret',
  oidcOrigin: 'http://localhost:8888/auth/realms/opentdf', exchange: 'client',
});
const client = new OpenTDF({authProvider, disableDPoP: true, platformUrl:'http://localhost:8080', policyEndpoint:'http://localhost:8080',defaultCreateOptions:{defaultKASEndpoint:'http://localhost:8080/kas'}});
try {
 const plaintext = await readFile(input);
 const publicKey = await readFile(publicKeyPath, "utf8");
 // Pinned discovery prefers the platform base key even when an algorithm is
 // requested. Seed its supported explicit kasKeys cache to pin both profiles.
 const attributeValues = [{fqn:attribute,kasKeys:[{kasUri:"http://localhost:8080/kas",publicKey:{algorithm:wrapping === "rsa:2048" ? 1 : 3,kid,pem:publicKey}}]}];
 // The pinned high-level OpenTDF facade drops metadata; its stock lower-level
 // client exposes the option and independently implements all wire/crypto work.
 const stream = await client.tdf3Client.encrypt({source:new Blob([plaintext]).stream(),scope:{attributes:attribute?[attribute]:[],attributeValues},defaultKASEndpoint:'http://localhost:8080/kas',autoconfigure:false,windowSize:16384,wrappingKeyAlgorithm:wrapping,splitPlan:[{kas:'http://localhost:8080/kas',kid}],segmentIntegrityAlgorithm:algorithm,metadata:metadata?JSON.parse(metadata):undefined});
 await writeFile(output,Buffer.from(await new Response(stream.stream).arrayBuffer()));
} finally { client.close(); }
