package rt

import nativeencoding "goalchemyout/cap/encoding"

func LibEncodingBase64URLEncode(data []byte) (string, error) {
	return nativeencoding.Base64URLEncode(data)
}
