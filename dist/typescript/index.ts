// Minimal host value adapter. TDF protocol executes only in compiler output.
import * as generated from './main.js';
export type { Config, KASRoute, EncryptOptions, Failure } from './main.js';
export interface Decrypted {
  Payload: Uint8Array;
  Metadata: Uint8Array;
  HasMetadata: boolean;
  ManifestJSON: string;
}
const utf8 = new TextDecoder('utf-8', { fatal: true });
function decode(s: unknown): string {
  if (typeof s !== 'string') return '';
  return utf8.decode(Uint8Array.from(s, (c) => c.charCodeAt(0)));
}
export interface AccessToken {
  value: string;
  scheme: string;
  expiresAt: bigint;
  confirmationJKT?: string;
}
export type TokenProvider = (signal: AbortSignal) => Promise<AccessToken>;
export interface CallOptions {
  signal?: AbortSignal;
  tokenProvider?: TokenProvider;
}
export const TokenProviderName = 'access-token';
export class TDFError extends Error {
  readonly kind: string;
  readonly code: string;
  readonly operation: string;
  readonly httpStatus: bigint;
  readonly serverCode: string;
  readonly serverMessage: string;
  readonly requiredObligations: string[];
  readonly causeCategory: string;
  constructor(error: generated.LibraryError) {
    const f = error.fields;
    super(
      'tdf: ' + decode(f.Operation) + ': ' + (f.Code === undefined ? error.kind : decode(f.Code)),
      { cause: error.cause },
    );
    this.name = 'TDFError';
    this.kind = error.kind;
    this.code = f.Code === undefined ? error.kind : decode(f.Code);
    this.operation = decode(f.Operation);
    this.httpStatus = typeof f.HTTPStatus === 'bigint' ? f.HTTPStatus : 0n;
    this.serverCode = decode(f.ServerCode);
    this.serverMessage = decode(f.ServerMessage);
    this.requiredObligations = Array.isArray(f.RequiredObligations)
      ? copyArray(f.RequiredObligations, decode)
      : [];
    this.causeCategory = decode(f.CauseCategory);
  }
}
function encode(s: string): string {
  const b = new TextEncoder().encode(s);
  let out = '';
  for (let i = 0; i < b.length; i += 8192) out += String.fromCharCode(...b.subarray(i, i + 8192));
  return out;
}
function copyArray<T, R>(a: readonly T[], convert: (value: T) => R): R[] {
  if (!Array.isArray(a) || a.length > 64 << 20)
    throw new generated.LibraryError('invalid_argument');
  const result: R[] = [];
  for (let i = 0; i < a.length; i++) result.push(convert(a[i]));
  return result;
}
function configValue(input: generated.Config): generated.Config {
  const c = { ...input };
  for (const key of Object.keys(c) as (keyof generated.Config)[]) {
    if (typeof c[key] === 'string') (c as Record<string, unknown>)[key] = encode(c[key] as string);
  }
  if (c.AllowedKAS)
    c.AllowedKAS = copyArray(c.AllowedKAS, (r) => ({
      URL: r.URL === undefined ? undefined : encode(r.URL),
      APIBaseURL: r.APIBaseURL === undefined ? undefined : encode(r.APIBaseURL),
    }));
  return c;
}
function optionValue(input: generated.EncryptOptions): generated.EncryptOptions {
  const o = { ...input };
  for (const key of ['PolicyBase64', 'SegmentHashAlgorithm', 'MimeType'] as const)
    if (o[key] !== undefined) o[key] = encode(o[key]!);
  if (o.Attributes) o.Attributes = copyArray(o.Attributes, encode);
  if (o.Dissem) o.Dissem = copyArray(o.Dissem, encode);
  return o;
}
function callOptions(c: generated.Config, options: CallOptions): generated.CallOptions {
  if (!options.tokenProvider) return { signal: options.signal };
  const provider = options.tokenProvider;
  c.TokenProviderName = TokenProviderName;
  return {
    signal: options.signal,
    callbacks: {
      [TokenProviderName]: async (signal) => {
        const token = await provider(signal);
        if (
          typeof token.expiresAt !== 'bigint' ||
          token.expiresAt < 0n ||
          token.expiresAt > 9223372036854775807n ||
          typeof token.value !== 'string' ||
          typeof token.scheme !== 'string' ||
          (token.confirmationJKT !== undefined && typeof token.confirmationJKT !== 'string')
        )
          throw new Error('invalid token provider result');
        return new TextEncoder().encode(
          JSON.stringify({
            value: token.value,
            scheme: token.scheme,
            expiresAt: token.expiresAt.toString(),
            confirmationJKT: token.confirmationJKT ?? '',
          }),
        );
      },
    },
  };
}
export async function encrypt(
  config: generated.Config,
  payload: Uint8Array,
  options: generated.EncryptOptions = {},
  call: CallOptions = {},
): Promise<Uint8Array> {
  try {
    const c = configValue(config);
    const opts = callOptions(c, call);
    const result = await generated.Encrypt(c, payload, optionValue(options), opts);
    if (result === null) throw new Error('invalid generated result');
    return result;
  } catch (e) {
    if (e instanceof generated.LibraryError) throw new TDFError(e);
    throw e;
  }
}
export async function decrypt(
  config: generated.Config,
  archive: Uint8Array,
  call: CallOptions = {},
): Promise<Decrypted> {
  try {
    const c = configValue(config);
    const opts = callOptions(c, call);
    const result = await generated.Decrypt(c, archive, opts);
    return {
      Payload: result.Payload ?? new Uint8Array(),
      Metadata: result.Metadata ?? new Uint8Array(),
      HasMetadata: result.HasMetadata!,
      ManifestJSON: utf8.decode(result.ManifestJSON!),
    };
  } catch (e) {
    if (e instanceof generated.LibraryError) throw new TDFError(e);
    throw e;
  }
}
