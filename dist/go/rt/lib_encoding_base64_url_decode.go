package rt

import nativeencoding "goalchemyout/cap/encoding"

func LibEncodingBase64URLDecode(data string) ([]byte, error) {
	return nativeencoding.Base64URLDecode(data)
}
