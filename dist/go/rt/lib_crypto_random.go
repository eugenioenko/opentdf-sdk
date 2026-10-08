package rt

import nativecrypto "goalchemyout/cap/crypto"

func LibCryptoRandom(t *Task, n int) {
	r0, r1 := nativecrypto.Random(n)
	t.RV = []any{r0, r1}
}
