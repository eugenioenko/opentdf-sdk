#ifndef OPENTDF_TDF3_H
#define OPENTDF_TDF3_H
#include "library.h"
typedef struct { const uint8_t *data; size_t length; } tdf3_bytes;
typedef struct {tdf3_bytes URL,APIBaseURL;} tdf3_kas_route;
typedef struct {
 tdf3_bytes PlatformURL,KASURL;const tdf3_kas_route *AllowedKAS;size_t AllowedKASLength;
 tdf3_bytes IssuerURL,TokenURL,ClientID,ClientSecret,TokenProviderName;
 bool AllowHTTP;int64_t TimeoutMillis;
 tdf3_bytes KASPublicKeyPEM,KID,KASAlgorithm,SessionAlgorithm,AuthPrivateKeyPEM,AuthAlgorithm;
 bool DPoP;
} tdf3_config;
typedef struct {
 tdf3_bytes PolicyBase64;const tdf3_bytes *Attributes;size_t AttributesLength;
 const tdf3_bytes *Dissem;size_t DissemLength;
 int64_t SegmentSize;bool HasSegmentSize;tdf3_bytes SegmentHashAlgorithm,MimeType,Metadata;bool IncludeMetadata;
} tdf3_encrypt_options;
typedef struct {uint8_t *data;size_t length;} tdf3_owned_bytes;
typedef struct {tdf3_owned_bytes Payload,Metadata,ManifestJSON;bool HasMetadata;} tdf3_result;
typedef struct {
 int Kind;tdf3_owned_bytes Message,Code,Operation,ServerCode,ServerMessage,CauseCategory;
 int64_t HTTPStatus;tdf3_owned_bytes *RequiredObligations;size_t RequiredObligationsLength;
} tdf3_error;
/* Kind 0 success, 1 declared source error, 2 source panic, 3 host fault,
 * 4 source fatal, 5 invalid native input, 6 canceled while queued. */
void tdf3_result_free(tdf3_result *);
void tdf3_error_free(tdf3_error *);
int tdf3_encrypt(const tdf3_config *,tdf3_bytes,const tdf3_encrypt_options *,const gxc_options *,tdf3_result *,tdf3_error *);
int tdf3_decrypt(const tdf3_config *,tdf3_bytes,const gxc_options *,tdf3_result *,tdf3_error *);
typedef struct tdf3_operation tdf3_operation;
/* Submission snapshots recursive inputs before returning. Provider state remains
 * caller-owned until operation destruction. Cancel queued/active calls; drive
 * polls/waits for owned completion, and only reports completion after all actual
 * native cleanup/joins. wait_millis=0 polls, -1 waits, positive waits bounded. */
tdf3_operation *tdf3_encrypt_submit(const tdf3_config *,tdf3_bytes,const tdf3_encrypt_options *,const gxc_options *,tdf3_error *);
tdf3_operation *tdf3_decrypt_submit(const tdf3_config *,tdf3_bytes,const gxc_options *,tdf3_error *);
void tdf3_operation_cancel(tdf3_operation *);
void tdf3_operation_wake(tdf3_operation *);
bool tdf3_operation_drive(tdf3_operation *,int64_t wait_millis);
/* Take transfers owned result/error once after completion. */
int tdf3_operation_take(tdf3_operation *,tdf3_result *,tdf3_error *);
/* Cancel + join + free; destroy(&op) accepts nil and sets op to nil. */
void tdf3_operation_destroy(tdf3_operation **);
#endif
