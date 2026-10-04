package rt

import nativeencoding "goalchemyout/cap/encoding"

func LibEncodingBase64Encode(data []byte) (string, error) { return nativeencoding.Base64Encode(data) }
