#!/usr/bin/env python3
"""Generate docs/parity-matrix.md from the reference API indices.

One row per exported symbol of the reference root package
(docs/reference-api.json) and of the in-scope subpackages
(docs/reference-subpackages.json). The status of each row comes from the
phase tables in docs/full-sdk-plan.md and the shared-source equivalent from
docs/api-spec.md; both are encoded below and reviewed when either document
changes. Rows are grouped by the reference file that declares the symbol.
"""
import json
import re
from collections import Counter, OrderedDict
from pathlib import Path

sdk = Path(__file__).resolve().parents[1]
lock = json.loads((sdk / 'references.lock.json').read_text())
platform = (sdk / lock['repositories']['platform']['path']).resolve()
api = json.loads((sdk / 'docs' / 'reference-api.json').read_text())
subs = json.loads((sdk / 'docs' / 'reference-subpackages.json').read_text())

IN_SCOPE = ['auth', 'auth/oauth', 'experimental/tdf', 'experimental/tdf/keysplit',
            'httputil', 'sdkconnect']
EXCLUDED = {'audit': 'service-side helpers, not client API (plan: Not planned)',
            'codegen/runner': 'build tooling, not client API (plan: Not planned)'}

NOT_PLANNED_PQ = 'not planned: post-quantum KAOs (goalchemy #21 closed)'

# Reference With* option -> api-spec field.
OPTION_FIELDS = {
    'WithClientCredentials': 'Options.ClientID/ClientSecret/Scopes',
    'WithTokenEndpoint': 'Options.TokenEndpoint',
    'WithTokenExchange': 'Options.TokenExchange',
    'WithPlatformConfiguration': 'Options.PlatformConfiguration',
    'WithConnectionValidation': 'Options.ValidateConnection',
    'WithInsecurePlaintextConn': 'Options.InsecurePlaintext',
    'WithInsecureSkipVerifyConn': 'Options.InsecureSkipVerify',
    'WithTLSCredentials': 'Options.TLS',
    'WithIPC': 'Options.UnixSocket (unsupported on Java and in browsers)',
    'WithCustomCoreConnection': 'Options.CoreEndpoint',
    'WithCustomPolicyConnection': 'Options.PolicyEndpoint',
    'WithCustomAuthorizationConnection': 'Options.AuthorizationEndpoint',
    'WithCustomEntityResolutionConnection': 'Options.EntityResolutionEndpoint',
    'WithCustomWellknownConnection': 'Options.WellknownEndpoint',
    'WithSessionEncryptionRSA': 'Options.SessionEncryptionRSAPEM',
    'WithSessionSignerRSA': 'Options.SessionSignerRSAPEM',
    'WithDPoPAlgorithm': 'Options.DPoPAlgorithm',
    'WithDPoPKeyPEM': 'Options.DPoPKeyPEM',
    'WithDPoPJWK': 'Options.DPoPKeyPEM (JWK converted by the host adapter)',
    'WithNoKIDInKAO': 'Options.NoKIDInKAO',
    'WithFulfillableObligationFQNs': 'Options.FulfillableObligationFQNs',
    'WithLogger': 'Options.LogLevel plus the host log sink',
    'WithOAuthAccessTokenSource': 'Options.TokenProviderName (host callback)',
    'WithKasInformation': 'TDFOptions.KasInformation',
    'WithDataAttributes': 'TDFOptions.Attributes',
    'WithDataAttributeValues': 'TDFOptions.AttributeValues',
    'WithAutoconfigure': 'TDFOptions.DisableAutoconfigure',
    'WithWrappingKeyAlg': 'TDFOptions.WrappingKeyAlgorithm',
    'WithSegmentSize': 'TDFOptions.SegmentSize',
    'WithInputSize': 'TDFOptions.InputSize',
    'WithMimeType': 'TDFOptions.MimeType',
    'WithMetaData': 'TDFOptions.Metadata',
    'WithAssertions': 'TDFOptions.Assertions',
    'WithSystemMetadataAssertion': 'TDFOptions.SystemMetadataAssertion',
    'WithTargetMode': 'TDFOptions.TargetMode',
    'WithPolicyFrom': 'TDFOptions.PolicyFrom',
    'WithAssertionVerificationKeys': 'ReaderOptions.AssertionVerificationKeys',
    'WithDisableAssertionVerification': 'ReaderOptions.DisableAssertionVerification',
    'WithKasAllowlist': 'ReaderOptions.KasAllowlist',
    'WithIgnoreAllowlist': 'ReaderOptions.IgnoreAllowlist',
    'WithMaxManifestSize': 'ReaderOptions.MaxManifestSize',
    'WithSchemaValidation': 'ReaderOptions.SchemaValidation',
    'WithSessionKeyType': 'ReaderOptions.SessionKeyType',
    'WithTDFFulfillableObligationFQNs': 'ReaderOptions.FulfillableObligationFQNs',
    'WithTDFs': 'BulkOptions.TDFs',
    'WithTDFType': 'BulkOptions.TDFType',
    'WithTDF3DecryptOptions': 'BulkOptions.Reader',
    'WithBulkKasAllowlist': 'BulkOptions.KasAllowlist',
    'WithBulkIgnoreAllowlist': 'BulkOptions.IgnoreAllowlist',
}

# Phase per reference option (plan phase tables).
OPTION_PHASES = {
    'WithClientCredentials': 2, 'WithTokenEndpoint': 2, 'WithPlatformConfiguration': 2,
    'WithConnectionValidation': 2, 'WithInsecurePlaintextConn': 2,
    'WithInsecureSkipVerifyConn': 2, 'WithLogger': 2, 'WithKasInformation': 2,
    'WithMimeType': 2, 'WithMetaData': 2, 'WithSegmentSize': 2, 'WithWrappingKeyAlg': 2,
    'WithSessionKeyType': 2, 'WithKasAllowlist': 2, 'WithIgnoreAllowlist': 2,
    'WithMaxManifestSize': 2, 'WithSessionEncryptionRSA': 2,
    'WithCustomCoreConnection': 3, 'WithCustomPolicyConnection': 3,
    'WithCustomAuthorizationConnection': 3, 'WithCustomEntityResolutionConnection': 3,
    'WithCustomWellknownConnection': 3,
    'WithAutoconfigure': 4, 'WithDataAttributes': 4, 'WithDataAttributeValues': 4,
    'WithNoKIDInKAO': 4,
    'WithAssertions': 5, 'WithSystemMetadataAssertion': 5,
    'WithAssertionVerificationKeys': 5, 'WithDisableAssertionVerification': 5,
    'WithTargetMode': 6, 'WithSchemaValidation': 6, 'WithPolicyFrom': 6, 'WithInputSize': 6,
    'WithDPoPAlgorithm': 8, 'WithDPoPJWK': 8, 'WithDPoPKeyPEM': 8,
    'WithOAuthAccessTokenSource': 8, 'WithTokenExchange': 8, 'WithTLSCredentials': 8,
    'WithIPC': 8, 'WithSessionSignerRSA': 8,
    'WithFulfillableObligationFQNs': 9, 'WithTDFFulfillableObligationFQNs': 9,
    'WithTDFs': 9, 'WithTDFType': 9, 'WithTDF3DecryptOptions': 9,
    'WithBulkKasAllowlist': 9, 'WithBulkIgnoreAllowlist': 9,
}

# Explicit root rows: symbol -> (status, equivalent). Status is 'phase N' or
# 'not planned: reason'.
ROOT = {
    'WithExtraClientOptions': ('not planned: Go-specific Connect client options type (plan: Not planned)',
                               'none; transport configuration lives in Options'),
    'New': ('phase 2', 'sdk.New(ctx, platformEndpoint, Options)'),
    'SupportedFeatures': ('phase 1', 'sdk.SupportedFeatures()'),
    'Version': ('phase 1', 'sdk.Version (plus ReferenceVersion)'),
    'TDFSpecVersion': ('phase 6', 'sdk.TDFSpecVersion'),
    'IsValidTdf': ('phase 2', 'sdk.IsValidTdf(io.ReadSeeker)'),
    'GetTdfType': ('phase 2', 'sdk.GetTdfType(io.ReadSeeker)'),
    'IsPlatformEndpointMalformed': ('phase 10', 'sdk.IsPlatformEndpointMalformed'),
    'KeyTypeToPolicyAlgorithm': ('phase 4', 'sdk.KeyTypeToPolicyAlgorithm'),
    'PolicyAlgorithmToKeyType': ('phase 4', 'sdk.PolicyAlgorithmToKeyType'),
    'NewAttributeNameFQN': ('phase 4', 'sdk.NewAttributeNameFQN'),
    'NewAttributeValueFQN': ('phase 4', 'sdk.NewAttributeValueFQN'),
    'NewResourceLocator': ('phase 4', 'sdk.NewResourceLocator'),
    'NewResourceLocatorFromReader': ('phase 10', 'sdk.NewResourceLocatorFromReader'),
    'DefaultKeySplitter': ('phase 4', 'sdk.DefaultKeySplitter'),
    'GetSystemMetadataAssertionConfig': ('phase 5', 'sdk.GetSystemMetadataAssertionConfig'),
    'FromBulkErrors': ('phase 9', 'sdk.FromBulkErrors'),
    'NewChunkedWriter': ('phase 10', 'SDK.NewChunkedWriter(ctx, ChunkedWriterOptions)'),
    'NewDPoPValidationHTTPClient': ('phase 8', 'sdk/auth DPoP transport (host HTTP client validation helper)'),
    'NewIDPAccessTokenSource': ('phase 8', 'sdk/auth client-credentials TokenSource'),
    'NewOAuthAccessTokenSource': ('phase 8', 'sdk/auth host-callback TokenSource'),
    'NewIDPTokenExchangeTokenSource': ('phase 8', 'sdk/auth token-exchange TokenSource'),
    'NewCertExchangeTokenSource': ('phase 8', 'sdk/auth certificate-exchange TokenSource'),
    'SDK.Conn': ('phase 3', 'no exported connection; service clients share the SDK transport'),
    'SDK.CreateTDF': ('phase 2', 'SDK.EncryptBytes (phase 2); SDK.CreateTDF(ctx, w, r, TDFOptions) streams in phase 7'),
    'SDK.CreateTDFContext': ('phase 2', 'SDK.CreateTDF takes ctx; streaming form in phase 7'),
    'SDK.LoadTDF': ('phase 2', 'SDK.LoadTDF(ctx, io.ReadSeeker, ReaderOptions)'),
    'SDK.DecryptBytes': ('phase 2', 'SDK.DecryptBytes(ctx, []byte, ReaderOptions)'),
    'SDK.DecryptTo': ('phase 7', 'SDK.DecryptTo(ctx, io.Writer, []byte, ReaderOptions)'),
    'SDK.DecryptFile': ('phase 7', 'SDK.DecryptFile(ctx, inputPath, outputPath, ReaderOptions)'),
    'SDK.GetBaseKey': ('phase 2', 'SDK.GetBaseKey'),
    'SDK.StoreKASKeys': ('phase 2', 'SDK.StoreKASKeys'),
    'SDK.IsHealthy': ('phase 2', 'SDK.IsHealthy'),
    'SDK.Close': ('phase 2', 'SDK.Close'),
    'SDK.Logger': ('phase 2', 'SDK.Logger'),
    'SDK.Auth': ('phase 3', 'SDK.Auth'),
    'SDK.ListAttributes': ('phase 3', 'SDK.ListAttributes'),
    'SDK.GetEntityAttributes': ('phase 3', 'SDK.GetEntityAttributes'),
    'SDK.ValidateAttributes': ('phase 3', 'SDK.ValidateAttributes'),
    'SDK.AttributeExists': ('phase 3', 'SDK.AttributeExists'),
    'SDK.AttributeValueExists': ('phase 3', 'SDK.AttributeValueExists'),
    'SDK.BulkDecrypt': ('phase 9', 'SDK.BulkDecrypt(ctx, BulkOptions)'),
    'SDK.PrepareBulkDecrypt': ('phase 9', 'SDK.PrepareBulkDecrypt(ctx, BulkOptions)'),
    'Reader.Init': ('phase 2', 'Reader.Init'),
    'Reader.Manifest': ('phase 2', 'Reader.Manifest'),
    'Reader.Policy': ('phase 2', 'Reader.Policy'),
    'Reader.DataAttributes': ('phase 2', 'Reader.DataAttributes'),
    'Reader.UnencryptedMetadata': ('phase 2', 'Reader.UnencryptedMetadata'),
    'Reader.Read': ('phase 7', 'Reader.Read (io.Reader)'),
    'Reader.ReadAt': ('phase 7', 'Reader.ReadAt (io.ReaderAt)'),
    'Reader.Seek': ('phase 7', 'Reader.Seek (io.Seeker)'),
    'Reader.WriteTo': ('phase 7', 'Reader.WriteTo (io.WriterTo)'),
    'Reader.UnsafePayloadKeyRetrieval': ('phase 7', 'Reader.UnsafePayloadKeyRetrieval'),
    'Reader.Obligations': ('phase 9', 'Reader.Obligations'),
    'TDFObject.Manifest': ('phase 2', 'TDFObject.Manifest'),
    'TDFObject.Size': ('phase 2', 'TDFObject.Size'),
    'AuthClient.AccessToken': ('phase 3', 'AuthClient.AccessToken'),
    'PlatformConfiguration.AuthzEndpoint': ('phase 2', 'PlatformConfiguration.AuthzEndpoint'),
    'PlatformConfiguration.Issuer': ('phase 2', 'PlatformConfiguration.Issuer'),
    'PlatformConfiguration.TokenEndpoint': ('phase 2', 'PlatformConfiguration.TokenEndpoint'),
    'AllowList.Add': ('phase 2', 'AllowList.Add'),
    'AllowList.IsAllowed': ('phase 2', 'AllowList.IsAllowed'),
    'Statement.UnmarshalJSON': ('phase 10', 'Statement JSON decoding quirks (object or string value)'),
    'BulkDecryptPrepared.BulkDecrypt': ('phase 9', 'BulkDecryptPrepared.BulkDecrypt'),
    'BulkErrors.Error': ('phase 9', 'Error{Kind: KindBulk}.Error'),
    'BulkErrors.Unwrap': ('phase 9', 'Error{Kind: KindBulk}.Unwrap ([]error)'),
    'ErrAssertionFailure.Error': ('phase 5', 'Error{Kind: KindAssertionFailure}.Error'),
    'ErrAssertionFailure.Unwrap': ('phase 5', 'Error{Kind: KindAssertionFailure}.Unwrap (ErrTampered)'),
    'Error.Error': ('phase 2', 'Error.Error'),
    'SplitResult.Validate': ('phase 4', 'SplitResult.Validate'),
    'SplitResult.VerifyReconstruction': ('phase 4', 'SplitResult.VerifyReconstruction'),
    'SigningAlgorithm.Valid': ('phase 8', 'SigningAlgorithm.Valid'),
    'AssertionKey.Algorithm': ('phase 5', 'AssertionKey.Algorithm'),
    'AssertionKey.IsEmpty': ('phase 5', 'AssertionKey.IsEmpty'),
    'AssertionVerificationKeys.Get': ('phase 5', 'AssertionVerificationKeys.Get'),
    'AssertionVerificationKeys.IsEmpty': ('phase 5', 'AssertionVerificationKeys.IsEmpty'),
    # Types.
    'SDK': ('phase 2', 'sdk.SDK handle (service client fields in phase 3)'),
    'Reader': ('phase 2', 'sdk.Reader handle (stream methods in phase 7)'),
    'TDFObject': ('phase 2', 'sdk.TDFObject'),
    'Option': ('phase 2', 'sdk.Options struct; With* adapters in hosts/go'),
    'TDFOption': ('phase 2', 'sdk.TDFOptions struct; With* adapters in hosts/go'),
    'TDFConfig': ('phase 2', 'sdk.TDFOptions'),
    'TDFReaderOption': ('phase 2', 'sdk.ReaderOptions struct; With* adapters in hosts/go'),
    'TDFReaderConfig': ('phase 2', 'sdk.ReaderOptions'),
    'KASClient': ('phase 2', 'unexported KAS client inside sdk; reached through SDK'),
    'KASKeyFetcher': ('phase 2', 'sdk.KASKeyFetcher'),
    'KASInfo': ('phase 2', 'sdk.KASInfo'),
    'AllowList': ('phase 2', 'sdk.AllowList'),
    'PlatformConfiguration': ('phase 2', 'sdk.PlatformConfiguration'),
    'TdfType': ('phase 2', 'sdk.TdfType'),
    'Error': ('phase 2', 'sdk.Error struct (Kind, Op, Message, ...)'),
    'AuthClient': ('phase 3', 'sdk.AuthClient'),
    'ConnectRPCConnection': ('phase 3', 'SDK transport plus Options endpoint overrides'),
    'AttributeNameFQN': ('phase 4', 'sdk.AttributeNameFQN'),
    'AttributeValueFQN': ('phase 4', 'sdk.AttributeValueFQN'),
    'KeySplitter': ('phase 4', 'sdk.KeySplitter (TDFOptions.KeySplitter)'),
    'Split': ('phase 4', 'sdk/tdf.Split'),
    'SplitResult': ('phase 4', 'sdk/tdf.SplitResult'),
    'KASPublicKey': ('phase 4', 'sdk/tdf.KASPublicKey'),
    'ResourceLocator': ('phase 4', 'sdk/tdf.ResourceLocator'),
    'Assertion': ('phase 5', 'sdk/tdf.Assertion'),
    'AssertionConfig': ('phase 5', 'sdk/tdf.AssertionConfig'),
    'AssertionKey': ('phase 5', 'sdk/tdf.AssertionKey'),
    'AssertionKeyAlg': ('phase 5', 'sdk/tdf.AssertionKeyAlg'),
    'AssertionType': ('phase 5', 'sdk/tdf.AssertionType'),
    'AssertionVerificationKeys': ('phase 5', 'sdk/tdf.AssertionVerificationKeys'),
    'AppliesToState': ('phase 5', 'sdk/tdf.AppliesToState'),
    'BindingMethod': ('phase 5', 'sdk/tdf.BindingMethod'),
    'Binding': ('phase 5', 'sdk/tdf.Binding'),
    'Scope': ('phase 5', 'sdk/tdf.Scope'),
    'Statement': ('phase 5', 'sdk/tdf.Statement'),
    'ErrAssertionFailure': ('phase 5', 'Error{Kind: KindAssertionFailure, AssertionID}'),
    'SchemaValidationIntensity': ('phase 6', 'sdk/tdf.SchemaValidationIntensity'),
    'RootIntegrityAlg': ('phase 6', 'sdk/tdf.RootIntegrityAlg'),
    'SegmentIntegrityAlg': ('phase 6', 'sdk/tdf.SegmentIntegrityAlg'),
    'IntegrityAlgorithm': ('phase 6', 'sdk/tdf.IntegrityAlgorithm'),
    'TDFFormat': ('phase 6', 'sdk/tdf.TDFFormat'),
    'SigningAlgorithm': ('phase 8', 'sdk.SigningAlgorithm'),
    'IDPAccessTokenSource': ('phase 8', 'sdk/auth client-credentials TokenSource'),
    'OAuthAccessTokenSource': ('phase 8', 'sdk/auth host-callback TokenSource'),
    'IDPTokenExchangeTokenSource': ('phase 8', 'sdk/auth token-exchange TokenSource'),
    'CertExchangeTokenSource': ('phase 8', 'sdk/auth certificate-exchange TokenSource'),
    'RequiredObligations': ('phase 9', 'sdk.RequiredObligations'),
    'BulkDecryptOption': ('phase 9', 'sdk.BulkOptions struct'),
    'BulkDecryptRequest': ('phase 9', 'sdk.BulkOptions'),
    'BulkDecryptPrepared': ('phase 9', 'sdk.BulkDecryptPrepared'),
    'BulkTDF': ('phase 9', 'sdk.BulkTDF'),
    'BulkErrors': ('phase 9', 'Error{Kind: KindBulk} with Cause unwrapping to []error'),
    'ChunkedWriter': ('phase 10', 'sdk.ChunkedWriter handle'),
    'ChunkedWriterOption': ('phase 10', 'sdk.ChunkedWriterOptions struct'),
    'ChunkedFinalizeOption': ('phase 10', 'sdk.ChunkedFinalizeOptions struct'),
    'ChunkedFinalizeResult': ('phase 10', 'sdk.ChunkedFinalizeResult'),
    'ChunkedSegmentResult': ('phase 10', 'sdk.ChunkedSegmentResult'),
}

MANIFEST_TYPES = ['Manifest', 'EncryptionInformation', 'KeyAccess', 'PolicyBinding', 'Method',
                  'IntegrityInformation', 'RootSignature', 'Segment', 'Payload', 'PolicyObject',
                  'EncryptedMetadata']
for name in MANIFEST_TYPES:
    ROOT[name] = ('phase 2', 'sdk/tdf.' + name + ' (plan lists it under phase 6; Reader.Manifest needs it in phase 2)')

ENUM_PHASE = {'AppliesToState': 5, 'AssertionKeyAlg': 5, 'AssertionType': 5, 'BindingMethod': 5,
              'Scope': 5, 'RootIntegrityAlg': 6, 'SegmentIntegrityAlg': 6, 'TdfType': 2}

VALUE_PHASE = {
    'SystemMetadataAssertionID': 5, 'SystemMetadataSchemaV1': 5,
    'ErrGrpcDialFailed': 2, 'ErrShutdownFailed': 2, 'ErrPlatformUnreachable': 2,
    'ErrHealthCheckUnsupported': 2, 'ErrPlatformConfigFailed': 2,
    'ErrPlatformEndpointMalformed': 2, 'ErrPlatformIssuerNotFound': 2,
    'ErrPlatformAuthzEndpointNotFound': 2, 'ErrPlatformTokenEndpointNotFound': 2,
    'ErrPlatformEndpointNotFound': 2, 'ErrAccessTokenInvalid': 8, 'ErrNoAccessTokenSource': 8,
    'ErrWellKnowConfigEmpty': 2, 'ErrAttributeNotFound': 3,
    'DefaultRSAKeySize': 2, 'ECKeySize256': 2, 'ECKeySize384': 4, 'ECKeySize521': 4,
    'JSONFormat': 6, 'XMLFormat': 6, 'HS256': 6, 'GMAC': 6,
    'TDFSpecVersion': 6, 'Version': 1,
    'ErrInvalid': 4, 'ErrInvalidPerSchema': 6,
    'ErrSplitterRequiresDefaultKAS': 4, 'ErrSplitterUnsupportedAlgorithm': 4,
    'Encrypted': 5, 'Unencrypted': 5, 'AssertionKeyAlgRS256': 5, 'AssertionKeyAlgHS256': 5,
    'HandlingAssertion': 5, 'BaseAssertion': 5, 'JWS': 5, 'RootHS256': 6,
    'Skip': 6, 'Lax': 6, 'Strict': 6, 'TrustedDataObjScope': 5, 'PayloadScope': 5,
    'SegmentHS256': 6, 'SegmentGMAC': 6,
    'ES256': 8, 'ES384': 8, 'ES512': 8, 'RS256': 8, 'RS384': 8, 'RS512': 8,
    'Invalid': 2, 'Standard': 2,
}

ERROR_KINDS = {
    'ErrTampered': 'KindTampered (every tamper kind unwraps to ErrTampered)',
    'ErrRootSigValidation': 'KindRootSignature', 'ErrSegSizeMismatch': 'KindSegmentSize',
    'ErrSegSizeUnresolved': 'KindSegmentSize', 'ErrSegSigValidation': 'KindSegmentSignature',
    'ErrGMACSignatureFailed': 'KindSegmentSignature', 'ErrTDFPayloadReadFail': 'KindPayloadRead',
    'ErrTDFPayloadInvalidOffset': 'KindInvalidOffset', 'ErrRootSignatureFailure': 'KindRootSignature',
    'ErrRewrapBadRequest': 'KindRewrapBadRequest',
    'ErrUnsupportedRootIntegrityAlgorithm': 'KindUnsupportedRootIntegrity',
    'ErrUnsupportedSegmentIntegrityAlgorithm': 'KindUnsupportedSegmentIntegrity',
    'ErrKASRequestError': 'KindKASRequest', 'ErrRewrapForbidden': 'KindRewrapForbidden',
    'ErrAESGCMExhausted': 'KindAESGCMExhausted',
    'ErrTDFNotDecryptable': 'KindNotDecryptable', 'ErrTDFDecryptFailed': 'KindDecryptFailed',
    'ErrBaseKeyNotFound': 'KindBaseKeyNotFound', 'ErrBaseKeyInvalidFormat': 'KindBaseKeyInvalid',
    'ErrBaseKeyEmpty': 'KindBaseKeyInvalid', 'ErrMarshalBaseKeyFailed': 'KindBaseKeyInvalid',
    'ErrUnmarshalBaseKeyFailed': 'KindBaseKeyInvalid',
    'ErrGrpcDialFailed': 'KindTransport', 'ErrShutdownFailed': 'KindTransport',
    'ErrPlatformUnreachable': 'KindTransport', 'ErrHealthCheckUnsupported': 'KindTransport',
    'ErrPlatformConfigFailed': 'KindTransport', 'ErrPlatformEndpointMalformed': 'KindInvalidInput',
    'ErrPlatformIssuerNotFound': 'KindInvalidInput', 'ErrPlatformAuthzEndpointNotFound': 'KindInvalidInput',
    'ErrPlatformTokenEndpointNotFound': 'KindInvalidInput', 'ErrPlatformEndpointNotFound': 'KindInvalidInput',
    'ErrAccessTokenInvalid': 'KindAuth', 'ErrNoAccessTokenSource': 'KindAuth',
    'ErrWellKnowConfigEmpty': 'KindTransport', 'ErrAttributeNotFound': 'KindInvalidInput',
    'ErrInvalid': 'KindInvalidInput (granter.go, phase 4)', 'ErrInvalidPerSchema': 'KindInvalidInput',
    'ErrSplitterRequiresDefaultKAS': 'KindInvalidInput', 'ErrSplitterUnsupportedAlgorithm': 'KindUnsupportedFeature',
}
for i, name in enumerate(['ErrChunkedAlreadyFinalized', 'ErrChunkedCleanupFailed', 'ErrChunkedCloseFailed',
                          'ErrChunkedFinalizeFailed', 'ErrChunkedInvalidSegmentIndex',
                          'ErrChunkedMissingSegmentZero', 'ErrChunkedSegmentAlreadyWritten',
                          'ErrChunkedWriteInFlight']):
    VALUE_PHASE[name] = 10
    ERROR_KINDS[name] = 'KindInvalidInput (chunked writer)'
for name in ['ErrAESGCMExhausted', 'ErrTampered', 'ErrRootSigValidation', 'ErrSegSizeMismatch',
             'ErrSegSizeUnresolved', 'ErrSegSigValidation', 'ErrGMACSignatureFailed',
             'ErrTDFPayloadReadFail', 'ErrTDFPayloadInvalidOffset', 'ErrRootSignatureFailure',
             'ErrRewrapBadRequest', 'ErrUnsupportedRootIntegrityAlgorithm',
             'ErrUnsupportedSegmentIntegrityAlgorithm', 'ErrKASRequestError', 'ErrRewrapForbidden',
             'ErrTDFNotDecryptable', 'ErrTDFDecryptFailed', 'ErrBaseKeyNotFound',
             'ErrBaseKeyInvalidFormat', 'ErrBaseKeyEmpty', 'ErrMarshalBaseKeyFailed',
             'ErrUnmarshalBaseKeyFailed']:
    VALUE_PHASE[name] = 2

SUBPACKAGE_PHASE = {'auth': 8, 'auth/oauth': 8, 'experimental/tdf': 10,
                    'experimental/tdf/keysplit': 4, 'httputil': 8, 'sdkconnect': 3}
SUBPACKAGE_EQUIV = {
    'auth': 'sdk/auth (token sources, DPoP proofs, signed request tokens)',
    'auth/oauth': 'sdk/auth built-in client-credentials, token-exchange and cert-exchange flows',
    'experimental/tdf': 'sdk/experimental/tdf',
    'experimental/tdf/keysplit': 'sdk/experimental/tdf/keysplit (ported with granter.go and key_splitter.go)',
    'httputil': 'Options.TLS/InsecureSkipVerify over lib/http.DoWith; no separate package',
    'sdkconnect': 'sdk/protocol/<service> generated Connect clients',
}
SUBPACKAGE_OVERRIDES = {
    'experimental/tdf': {
        'WithAttributeValues': ('phase 10', 'experimental writer option; pure ML-KEM/X-Wing KAOs rejected'),
    },
    'keysplit': {},
}


def sources(rel):
    return sorted(p for p in (platform / rel).glob('*.go') if not p.name.endswith('_test.go'))


def declaring_file(rel, kind, sym):
    recv = None
    name = sym
    if '.' in sym:
        recv, name = sym.split('.', 1)
    if kind == 'func':
        if recv:
            pat = re.compile(r'^func \([^)]*\*?' + re.escape(recv) + r'\) ' + re.escape(name) + r'[\[(]', re.M)
        else:
            pat = re.compile(r'^func ' + re.escape(name) + r'[\[(]', re.M)
        pats = [pat]
    elif kind == 'type':
        pats = [re.compile(r'^type ' + re.escape(name) + r'\b', re.M),
                re.compile(r'^\t' + re.escape(name) + r' +(struct|interface|func|map|\[\]|string|int|=)', re.M)]
    else:
        pats = [re.compile(r'^(\t| )*' + re.escape(name) + r'\b[^\n=]*=', re.M),
                re.compile(r'^(const|var) ' + re.escape(name) + r'\b', re.M)]
    for pat in pats:
        hits = [p.name for p in sources(rel) if pat.search(p.read_text())]
        if hits:
            return hits[0]
    return None


# Symbols delivered by a merged phase PR: their rows read `done`. Phase 2
# part 1 (handle API, discovery, Go).
DONE = {
    'New', 'IsValidTdf', 'GetTdfType', 'IsPlatformEndpointMalformed',
    'KeyTypeToPolicyAlgorithm', 'PolicyAlgorithmToKeyType',
    'SDK.CreateTDF', 'SDK.CreateTDFContext', 'SDK.LoadTDF', 'SDK.DecryptBytes',
    'SDK.GetBaseKey', 'SDK.StoreKASKeys', 'SDK.IsHealthy', 'SDK.Close', 'SDK.Logger',
    'Reader.Init', 'Reader.Manifest', 'Reader.Policy', 'Reader.DataAttributes',
    'Reader.UnencryptedMetadata', 'TDFObject.Manifest', 'TDFObject.Size',
    'PlatformConfiguration.AuthzEndpoint', 'PlatformConfiguration.Issuer',
    'PlatformConfiguration.TokenEndpoint', 'AllowList.Add', 'AllowList.IsAllowed',
    'Error.Error', 'TdfType.String', 'SigningAlgorithm.Valid',
    'RootIntegrityAlg.String', 'SegmentIntegrityAlg.String',
    'SDK', 'Reader', 'TDFObject', 'Option', 'TDFOption', 'TDFConfig', 'TDFReaderOption',
    'TDFReaderConfig', 'KASClient', 'KASKeyFetcher', 'KASInfo', 'AllowList',
    'PlatformConfiguration', 'TdfType', 'Error', 'SigningAlgorithm',
    'RootIntegrityAlg', 'SegmentIntegrityAlg',
    'Manifest', 'EncryptionInformation', 'KeyAccess', 'PolicyBinding', 'Method',
    'IntegrityInformation', 'RootSignature', 'Segment', 'Payload', 'PolicyObject',
    'EncryptedMetadata',
    'WithClientCredentials', 'WithTokenEndpoint', 'WithPlatformConfiguration',
    'WithConnectionValidation', 'WithInsecurePlaintextConn', 'WithInsecureSkipVerifyConn',
    'WithLogger', 'WithKasInformation', 'WithMimeType', 'WithMetaData', 'WithSegmentSize',
    'WithWrappingKeyAlg', 'WithSessionKeyType', 'WithKasAllowlist', 'WithIgnoreAllowlist',
    'WithMaxManifestSize', 'WithSessionEncryptionRSA', 'WithSessionSignerRSA',
    'WithDPoPAlgorithm', 'WithDPoPKeyPEM', 'WithNoKIDInKAO', 'WithInputSize',
    'WithDisableAssertionVerification',
    'ErrGrpcDialFailed', 'ErrShutdownFailed', 'ErrPlatformUnreachable',
    'ErrHealthCheckUnsupported', 'ErrPlatformConfigFailed', 'ErrPlatformEndpointMalformed',
    'ErrPlatformIssuerNotFound', 'ErrPlatformAuthzEndpointNotFound',
    'ErrPlatformTokenEndpointNotFound', 'ErrPlatformEndpointNotFound',
    'ErrNoAccessTokenSource', 'ErrWellKnowConfigEmpty',
    'DefaultRSAKeySize', 'ECKeySize256', 'ECKeySize384', 'ECKeySize521',
    'Invalid', 'Standard', 'RootHS256', 'SegmentHS256', 'SegmentGMAC',
    'ES256', 'ES384', 'ES512', 'RS256', 'RS384', 'RS512',
    'ErrAESGCMExhausted', 'ErrTampered', 'ErrRootSigValidation', 'ErrSegSizeMismatch',
    'ErrSegSizeUnresolved', 'ErrSegSigValidation', 'ErrGMACSignatureFailed',
    'ErrTDFPayloadReadFail', 'ErrTDFPayloadInvalidOffset', 'ErrRootSignatureFailure',
    'ErrRewrapBadRequest', 'ErrUnsupportedRootIntegrityAlgorithm',
    'ErrUnsupportedSegmentIntegrityAlgorithm', 'ErrKASRequestError', 'ErrRewrapForbidden',
    'ErrTDFNotDecryptable', 'ErrTDFDecryptFailed', 'ErrBaseKeyNotFound',
    'ErrBaseKeyInvalidFormat', 'ErrBaseKeyEmpty', 'ErrMarshalBaseKeyFailed',
    'ErrUnmarshalBaseKeyFailed',
}

def root_status(kind, sym):
    status, equiv = root_status_planned(kind, sym)
    if sym in DONE:
        return ('done', equiv)
    return status, equiv


def root_status_planned(kind, sym):
    if sym in ROOT:
        return ROOT[sym]
    if sym in OPTION_FIELDS:
        phase = OPTION_PHASES[sym]
        return ('phase %d' % phase, OPTION_FIELDS[sym])
    if sym.startswith('WithChunked'):
        return ('phase 10', 'ChunkedWriterOptions field (named in phase 10)')
    if kind == 'func':
        recv, _, name = sym.partition('.')
        if name == 'String' and recv in ENUM_PHASE:
            return ('phase %d' % ENUM_PHASE[recv], 'sdk/tdf.%s.String' % recv)
        if recv in ('AttributeNameFQN', 'AttributeValueFQN', 'ResourceLocator'):
            return ('phase 4', 'sdk.%s' % sym)
        if recv == 'Assertion':
            return ('phase 5', 'sdk/tdf.%s' % sym)
        if recv in ('IDPAccessTokenSource', 'OAuthAccessTokenSource',
                    'IDPTokenExchangeTokenSource', 'CertExchangeTokenSource'):
            return ('phase 8', 'sdk/auth TokenSource.Token')
    if kind in ('const', 'var'):
        phase = VALUE_PHASE.get(sym)
        if phase is None:
            raise SystemExit('no phase for value ' + sym)
        if sym in ERROR_KINDS:
            return ('phase %d' % phase, 'sdk.%s sentinel; Error.Kind = %s' % (sym, ERROR_KINDS[sym]))
        return ('phase %d' % phase, 'sdk.%s' % sym if phase in (1, 2, 3) else 'sdk/tdf.%s' % sym)
    raise SystemExit('no status for %s %s' % (kind, sym))


def sdkconnect_equiv(kind, sym):
    if kind == 'type':
        base = sym.replace('ConnectWrapper', '')
        if sym.endswith('ConnectWrapper'):
            return 'sdk/protocol generated client handle for %s' % base
        return 'sdk/protocol generated %s (Connect unary methods)' % base
    recv, _, name = sym.partition('.')
    if name:
        return 'sdk/protocol %s.%s' % (recv.replace('ConnectWrapper', ''), name)
    return 'sdk/protocol constructor; clients are created by New and reached through SDK'


rows = []  # (package, file, kind, symbol, status, equivalent)
for f in api['functions']:
    kind = 'method' if '.' in f['symbol'] else 'func'
    status, equiv = root_status('func', f['symbol'])
    rows.append(('sdk', declaring_file('sdk', 'func', f['symbol']), kind, f['symbol'], status, equiv))
for t in api['types']:
    status, equiv = root_status('type', t['symbol'])
    rows.append(('sdk', declaring_file('sdk', 'type', t['symbol']), 'type', t['symbol'], status, equiv))
for block in api['values']:
    block_file = None
    for sym in block['symbols']:
        file = declaring_file('sdk', block['kind'], sym) or block_file
        block_file = block_file or file
        status, equiv = root_status(block['kind'], sym)
        rows.append(('sdk', file, block['kind'], sym, status, equiv))
for client in api['service_clients']:
    rows.append(('sdk', 'sdk.go', 'field', 'SDK.' + client['field'], 'phase 3',
                 'SDK.%s() accessor returning the generated %s' % (client['field'], client['go_type'])))

for pkg in subs['packages']:
    rel = pkg['source_directory']
    short = rel[len('sdk/'):]
    if short not in IN_SCOPE:
        continue
    phase = SUBPACKAGE_PHASE[short]
    lines = pkg['public_api_document'].splitlines()
    i = 0
    block_file = None
    while i < len(lines):
        line = lines[i]
        if line.startswith('func '):
            m = re.match(r'func \(([^)]+)\) (\w+)', line)
            f = re.match(r'func (\w+)', line)
            sym = (m.group(1).split()[-1].lstrip('*') + '.' + m.group(2)) if m else f.group(1)
            kind = 'method' if m else 'func'
            override = SUBPACKAGE_OVERRIDES.get(short, {}).get(sym)
            status, equiv = override or ('phase %d' % phase,
                                         sdkconnect_equiv('func', sym) if short == 'sdkconnect'
                                         else SUBPACKAGE_EQUIV[short] + ': ' + sym)
            rows.append((short, declaring_file(rel, 'func', sym), kind, sym, status, equiv))
        t = re.match(r'type (\w+) (.*)', line)
        if t:
            sym = t.group(1)
            override = SUBPACKAGE_OVERRIDES.get(short, {}).get(sym)
            status, equiv = override or ('phase %d' % phase,
                                         sdkconnect_equiv('type', sym) if short == 'sdkconnect'
                                         else SUBPACKAGE_EQUIV[short] + ': ' + sym)
            rows.append((short, declaring_file(rel, 'type', sym), 'type', sym, status, equiv))
        v = re.match(r'(const|var) (.*)', line)
        if v:
            syms = []
            if v.group(2) == '(':
                j = i + 1
                while j < len(lines) and lines[j] != ')':
                    mm = re.match(r'\s*([A-Z]\w*)', lines[j])
                    if mm:
                        syms.append(mm.group(1))
                    j += 1
            else:
                mm = re.match(r'([A-Z]\w*)', v.group(2))
                if mm:
                    syms.append(mm.group(1))
            block_file = None
            for sym in syms:
                file = declaring_file(rel, v.group(1), sym) or block_file
                block_file = block_file or file
                rows.append((short, file, v.group(1), sym, 'phase %d' % phase,
                             SUBPACKAGE_EQUIV[short] + ': ' + sym))
        i += 1

missing = [r for r in rows if r[1] is None]
if missing:
    raise SystemExit('no declaring file for: ' + ', '.join(r[3] for r in missing))

counts = Counter()
for r in rows:
    status = r[4]
    counts['not planned' if status.startswith('not planned') else status] += 1

lines = []
lines.append('# Parity matrix')
lines.append('')
lines.append('One row per exported symbol of the reference `platform/sdk` root package')
lines.append('([reference-api.json](reference-api.json), revision `%s`)' % api['reference_revision'][:12])
lines.append('and of the in-scope subpackages `auth`, `auth/oauth`, `experimental/tdf`,')
lines.append('`experimental/tdf/keysplit`, `httputil` and `sdkconnect`')
lines.append('([reference-subpackages.json](reference-subpackages.json)). The `audit` and')
lines.append('`codegen/runner` packages are excluded as build tooling and service-side')
lines.append('helpers ([full-sdk-plan.md](full-sdk-plan.md), "Not planned").')
lines.append('')
lines.append('Columns: the reference symbol, its kind, the shared-source equivalent named')
lines.append('in [api-spec.md](api-spec.md), and the status: `done`, `phase N` (the phase')
lines.append('of [full-sdk-plan.md](full-sdk-plan.md) that delivers it) or')
lines.append('`not planned: <reason>`. Rows are grouped by the reference file that')
lines.append('declares the symbol. The table is generated by `scripts/parity-matrix.py`,')
lines.append('which encodes the phase assignments; edit the script, not the table.')
lines.append('')
lines.append('A row turns `done` in the PR that delivers its symbol in the shared source (the')
lines.append('`DONE` set in the script). Phase 2 part 1 delivered the handle API, platform')
lines.append('discovery and the generated Go package; the symbols the plan lists under Phase 2')
lines.append('are all `done` in the shared source; part 2 exposes them through the generated')
lines.append('TypeScript and Python packages and part 3 through the Java, C#, Rust, C and Swift ones.')
lines.append('`ErrInvalid` moved to phase 4: it is declared in `granter.go`.')
lines.append('')
lines.append('Goalchemy issues opened for the gaps the phases depend on, detailed in')
lines.append('[goalchemy-gaps.md](goalchemy-gaps.md):')
lines.append('[#71](https://github.com/eugenioenko/goalchemy/issues/71) `std/net/url` (phase 2; released in v0.6.3 and pinned),')
lines.append('[#72](https://github.com/eugenioenko/goalchemy/issues/72) RFC 8785 JSON canonicalisation (phase 5),')
lines.append('[#73](https://github.com/eugenioenko/goalchemy/issues/73) `std/os` file handles, `CreateTemp` and `Rename` (phase 7),')
lines.append('[#74](https://github.com/eugenioenko/goalchemy/issues/74) `lib/runtime` host OS, architecture and version (phase 5),')
lines.append('[#75](https://github.com/eugenioenko/goalchemy/issues/75) bounded JSON Schema validator (phase 6),')
lines.append('[#76](https://github.com/eugenioenko/goalchemy/issues/76) X.509 certificate public key for `x5c` (phase 5).')
lines.append('')
lines.append('## Summary')
lines.append('')
lines.append('| Status | Symbols |')
lines.append('| --- | --- |')
total = 0
for key in ['done'] + ['phase %d' % n for n in range(1, 12)] + ['not planned']:
    if key in counts or key == 'done':
        lines.append('| `%s` | %d |' % (key, counts.get(key, 0)))
        total += counts.get(key, 0)
lines.append('| total | %d |' % total)
lines.append('')
per_pkg = Counter(r[0] for r in rows)
lines.append('| Package | Rows |')
lines.append('| --- | --- |')
for pkg in ['sdk'] + IN_SCOPE:
    lines.append('| `%s` | %d |' % (pkg, per_pkg[pkg]))
lines.append('')
lines.append('Phase assignments that the plan leaves open or states twice are recorded in')
lines.append('[progress.md](progress.md) under "Full SDK plan, Phase 0".')
lines.append('')

for pkg in ['sdk'] + IN_SCOPE:
    lines.append('## `%s`' % ('platform/sdk' if pkg == 'sdk' else 'platform/sdk/' + pkg))
    lines.append('')
    pkg_rows = [r for r in rows if r[0] == pkg]
    files = OrderedDict()
    for r in sorted(pkg_rows, key=lambda r: (r[1], r[3])):
        files.setdefault(r[1], []).append(r)
    for file, frows in files.items():
        lines.append('### `%s`' % file)
        lines.append('')
        lines.append('| Symbol | Kind | Shared-source equivalent | Status |')
        lines.append('| --- | --- | --- | --- |')
        for r in frows:
            lines.append('| `%s` | %s | %s | `%s` |' % (r[3], r[2], r[5], r[4]))
        lines.append('')

(sdk / 'docs' / 'parity-matrix.md').write_text('\n'.join(lines).rstrip('\n') + '\n')
print('rows', len(rows), dict(counts))
