package rt

import nativecrypto "goalchemyout/cap/crypto"

func LibCryptoSHA256(t *Task, data []byte) {
	r0, r1 := nativecrypto.SHA256(data)
	t.RV = []any{r0, r1}
}
