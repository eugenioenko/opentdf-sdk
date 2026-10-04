package rt

import nativecrypto "goalchemyout/cap/crypto"

func LibCryptoAES256GCMEncrypt(t *Task, key []byte, nonce []byte, data []byte, aad []byte) {
	r0, r1 := nativecrypto.AES256GCMEncrypt(key, nonce, data, aad)
	t.RV = []any{r0, r1}
}
