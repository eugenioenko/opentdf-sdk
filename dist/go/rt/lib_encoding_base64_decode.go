package rt

import nativeencoding "goalchemyout/cap/encoding"

func LibEncodingBase64Decode(data string) ([]byte, error) { return nativeencoding.Base64Decode(data) }
