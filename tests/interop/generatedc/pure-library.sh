#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
GOAL="$SDK/../goalchemy"
BASE=${TDF_C_PURE_OUT:-"$SDK/.local/c-tdf-library/pure"}
mkdir -p "$BASE/source" "$BASE/consumer" "$BASE/scalar-source"
printf 'module purevalue\n\ngo 1.25\nrequire github.com/eugenioenko/goalchemy v0.0.0\nreplace github.com/eugenioenko/goalchemy => %s\n' "$GOAL" > "$BASE/source/go.mod"
cat > "$BASE/source/plain.go" <<'GO'
package plain
import "github.com/eugenioenko/goalchemy/lib/errors"
type Packet struct { Payload []byte; Count int64; Nested []Packet }
func Echo(input Packet) Packet { return input }
func Cycle() Packet { p:=make([]Packet,1);p[0].Nested=p;return p[0] }
func Panic() { panic("native\x00\xffpanic") }
func Checked(input []byte) ([]byte,error) { if len(input)==0{return nil,errors.New("empty")};return input,nil }
GO
(cd "$BASE/source"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/c-tdf-library/goalchemy" compile -gate cooperative -target c -out "$BASE/emitted" .)
GOALCHEMY_BDWGC="$GOAL/.toolchains/bdwgc" sh "$BASE/emitted/build.sh"
cat > "$BASE/consumer/main.c" <<'C'
#include "goalchemy.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void){
 uint8_t data[]={0,255,128,0};char *names[]={"Payload","Count"};
 gxc_value fields[]={{.kind=GXC_BYTES,.bytes=data,.length=4},{.kind=GXC_INT,.integer=INT64_MAX}},arg={.kind=GXC_RECORD,.names=names,.items=fields,.length=2},out={0};gxc_error err={0};
 for(int i=0;i<40;i++){assert(!goalchemy_invoke("Echo",&arg,1,NULL,&out,&err));assert(gxc_field(&out,"Count")->integer==INT64_MAX);assert(gxc_field(&out,"Payload")->length==4);assert(!memcmp(gxc_field(&out,"Payload")->bytes,data,4));gxc_value_free(&out);gxc_error_free(&err);}
 assert(goalchemy_invoke("Panic",NULL,0,NULL,&out,&err)==2);assert(err.message.length>10);assert(memchr(err.message.bytes,0,err.message.length));gxc_error_free(&err);
 gxc_value empty={.kind=GXC_BYTES};assert(goalchemy_invoke("Checked",&empty,1,NULL,&out,&err)==1);gxc_error_free(&err);
 assert(!goalchemy_invoke("Checked",fields,1,NULL,&out,&err));assert(out.length==4);gxc_value_free(&out);
 gxc_value huge={.kind=GXC_LIST,.length=(size_t)UINT32_MAX+1};assert(goalchemy_invoke("Echo",&huge,1,NULL,&out,&err)==5);gxc_error_free(&err);
 assert(goalchemy_invoke("Cycle",NULL,0,NULL,&out,&err)==3);assert(!out.items);gxc_error_free(&err);
 assert(!goalchemy_invoke("Echo",&arg,1,NULL,&out,&err));gxc_value_free(&out);gxc_value_free(&out);gxc_error_free(&err);gxc_error_free(&err);
 puts("PASS independent crypto-free value library: 40 owned exact-int/binary calls, panic/error recovery, overflow and idempotent frees");
}
C
${CC:-cc} ${CFLAGS:--O2} -std=c17 -Wall -Wextra -Werror -I"$BASE/emitted" "$BASE/consumer/main.c" "$BASE/emitted/libgoalchemy.a" "$GOAL/.toolchains/bdwgc/lib/libgc.a" -lpthread -ldl -o "$BASE/consumer/value"
"$BASE/consumer/value"
printf 'module scalar\n\ngo 1.25\nrequire github.com/eugenioenko/goalchemy v0.0.0\nreplace github.com/eugenioenko/goalchemy => %s\n' "$GOAL" > "$BASE/scalar-source/go.mod"
printf 'package scalar\nfunc Add(a,b int64) int64 {return a+b}\n' > "$BASE/scalar-source/scalar.go"
(cd "$BASE/scalar-source"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/c-tdf-library/goalchemy" compile -target c -out "$BASE/scalar" .)
GOALCHEMY_BDWGC="$GOAL/.toolchains/bdwgc" sh "$BASE/scalar/build.sh"
cat > "$BASE/consumer/scalar.c" <<'C'
#include "goalchemy.h"
#include <assert.h>
int main(void){int64_t r;assert(!goalchemy_init());assert(!scalar_Add(20,22,&r));assert(r==42);return 0;}
C
${CC:-cc} ${CFLAGS:--O2} -std=c17 -Wall -Wextra -Werror -I"$BASE/scalar" "$BASE/consumer/scalar.c" "$BASE/scalar/libgoalchemy.a" "$GOAL/.toolchains/bdwgc/lib/libgc.a" -lpthread -ldl -o "$BASE/consumer/scalar"
"$BASE/consumer/scalar"
printf 'PASS independent accepted legacy scalar API\n'
