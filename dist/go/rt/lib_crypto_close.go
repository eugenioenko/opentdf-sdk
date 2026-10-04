package rt

import nativecrypto "goalchemyout/cap/crypto"

type Key = nativecrypto.Key

func LibCryptoClose(t *Task, key *Key) { key.Close(); t.RV = nil }
