package sdk

import (
	"context"
	"crypto/aes"
	"crypto/cipher"
	"crypto/ecdh"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
)

// Reference TDF fixtures from platform/sdk/sdk_test.go.
const (
	goodStandardTdf       = "UEsDBC0ACAAAAJ2TFTEAAAAAAAAAAAAAAAAJAAAAMC5wYXlsb2Fktu4m+vdwl0mtjhY3U5e7TG2o1s8ifK+RAhFNjRjGTLJ7V3w5UEsHCGiY7skkAAAAJAAAAFBLAwQtAAgAAACdkxUxAAAAAAAAAAAAAAAADwAAADAubWFuaWZlc3QuanNvbnsiZW5jcnlwdGlvbkluZm9ybWF0aW9uIjp7InR5cGUiOiJzcGxpdCIsInBvbGljeSI6ImV5SjFkV2xrSWpvaU1HTTFORGsyWlRZdE5EYzRaaTB4TVdWbUxXSXlOakV0WWpJMVl6UmhORE14TjJFM0lpd2lZbTlrZVNJNmV5SmtZWFJoUVhSMGNtbGlkWFJsY3lJNlczc2lZWFIwY21saWRYUmxJam9pYUhSMGNITTZMeTlsZUdGdGNHeGxMbU52YlM5aGRIUnlMMkYwZEhJeEwzWmhiSFZsTDNaaGJIVmxNU0lzSW1ScGMzQnNZWGxPWVcxbElqb2lJaXdpYVhORVpXWmhkV3gwSWpwbVlXeHpaU3dpY0hWaVMyVjVJam9pSWl3aWEyRnpWVkpNSWpvaUluMWRMQ0prYVhOelpXMGlPbHRkZlgwPSIsImtleUFjY2VzcyI6W3sidHlwZSI6IndyYXBwZWQiLCJ1cmwiOiJodHRwOi8vbG9jYWxob3N0OjgwODAiLCJwcm90b2NvbCI6ImthcyIsIndyYXBwZWRLZXkiOiJ0VVMvUE9TaVBtOGV6OGhyL2dMVGN6Y1lOT0trcUNEclZiQTBWdHZna29QbHB0M1BDZVpTdDNndnlQNVZKZXBNMmNqdVBhUWJJUGlyMjlWdVJ2T1RXZmQzRUh1KzgyVCtFNEVZbEpBM25VbDdGQTRMUGZhUEtXWk1zTExHUkJJVUxZT0VhMWJma1MvUm9Xb0EwK283WlFFVkNhYmdJN2JFRDJKV2Q2aG1yam1iUnM2d0lwOVFXNUs4Q3dJWjZVZjlGMXEwRDViTmlrbGxHaCtiaVJsV1NucEwxbHBPaFdva1gxdUJsU0VRSDNvM2JtVXFTNVVaUjRmYUxuTW5xOGR0bS8wYnJjTjUwaFNiK0xTTlZkd2daTEszTTRHTmxEeGdzcDkxY0VuYjZoZktLemdSY0VCS0tMQTF1b3BXNHdCRG9BamFuWWplQlZVT3ZBZEI5ek45T3c9PSIsInBvbGljeUJpbmRpbmciOnsiYWxnIjoiSFMyNTYiLCJoYXNoIjoiWmpBek1HWXlZekl4WlRCbU16Tm1NamhoTWpGalpqSTJaRE5oWlRrMk5ERTNaREJoWlRrM05ESTJNREExTnpVMU1UVTFNV0ZpTTJSak9EUTFabU0yWWc9PSJ9LCJraWQiOiJyMSJ9XSwibWV0aG9kIjp7ImFsZ29yaXRobSI6IkFFUy0yNTYtR0NNIiwiaXYiOiIiLCJpc1N0cmVhbWFibGUiOnRydWV9LCJpbnRlZ3JpdHlJbmZvcm1hdGlvbiI6eyJyb290U2lnbmF0dXJlIjp7ImFsZyI6IkhTMjU2Iiwic2lnIjoiWkdWaFltRmtNRGhsTURCbU1UVm1ZekJtTVdFME0ySmhOamhrTmpBMVpUazFNVGRtWmpoa1pETmtNekk0Tldaa01XUXhOVFZsWXpjME1EVXhPRE13Tmc9PSJ9LCJzZWdtZW50SGFzaEFsZyI6IkdNQUMiLCJzZWdtZW50U2l6ZURlZmF1bHQiOjIwOTcxNTIsImVuY3J5cHRlZFNlZ21lbnRTaXplRGVmYXVsdCI6MjA5NzE4MCwic2VnbWVudHMiOlt7Imhhc2giOiJNakkzWTJGbU9URXdNakV4TkdRNFpERTRZelkwWTJJeU4ySTFOemRqTXprPSIsInNlZ21lbnRTaXplIjo4LCJlbmNyeXB0ZWRTZWdtZW50U2l6ZSI6MzZ9XX19LCJwYXlsb2FkIjp7InR5cGUiOiJyZWZlcmVuY2UiLCJ1cmwiOiIwLnBheWxvYWQiLCJwcm90b2NvbCI6InppcCIsIm1pbWVUeXBlIjoiYXBwbGljYXRpb24vb2N0ZXQtc3RyZWFtIiwiaXNFbmNyeXB0ZWQiOnRydWV9fVBLBwgwpFOlrwUAAK8FAABQSwECLQAtAAgAAACdkxUxaJjuySQAAAAkAAAACQAAAAAAAAAAAAAAAAAAAAAAMC5wYXlsb2FkUEsBAi0ALQAIAAAAnZMVMTCkU6WvBQAArwUAAA8AAAAAAAAAAAAAAAAAWwAAADAubWFuaWZlc3QuanNvblBLBQYAAAAAAgACAHQAAABHBgAAAAA="
	specNamedManifestTdf  = "UEsDBBQACAAAAAAAAAAAAAAAAAAAAAAAAAAJAAAAMC5wYXlsb2Fktu4m+vdwl0mtjhY3U5e7TG2o1s8ifK+RAhFNjRjGTLJ7V3w5UEsHCGiY7skkAAAAJAAAAFBLAwQUAAgAAAAAAAAAAAAAAAAAAAAAAAAADQAAAG1hbmlmZXN0Lmpzb257ImVuY3J5cHRpb25JbmZvcm1hdGlvbiI6eyJ0eXBlIjoic3BsaXQiLCJwb2xpY3kiOiJleUoxZFdsa0lqb2lNR00xTkRrMlpUWXRORGM0WmkweE1XVm1MV0l5TmpFdFlqSTFZelJoTkRNeE4yRTNJaXdpWW05a2VTSTZleUprWVhSaFFYUjBjbWxpZFhSbGN5STZXM3NpWVhSMGNtbGlkWFJsSWpvaWFIUjBjSE02THk5bGVHRnRjR3hsTG1OdmJTOWhkSFJ5TDJGMGRISXhMM1poYkhWbEwzWmhiSFZsTVNJc0ltUnBjM0JzWVhsT1lXMWxJam9pSWl3aWFYTkVaV1poZFd4MElqcG1ZV3h6WlN3aWNIVmlTMlY1SWpvaUlpd2lhMkZ6VlZKTUlqb2lJbjFkTENKa2FYTnpaVzBpT2x0ZGZYMD0iLCJrZXlBY2Nlc3MiOlt7InR5cGUiOiJ3cmFwcGVkIiwidXJsIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwIiwicHJvdG9jb2wiOiJrYXMiLCJ3cmFwcGVkS2V5IjoidFVTL1BPU2lQbThlejhoci9nTFRjemNZTk9La3FDRHJWYkEwVnR2Z2tvUGxwdDNQQ2VaU3QzZ3Z5UDVWSmVwTTJjanVQYVFiSVBpcjI5VnVSdk9UV2ZkM0VIdSs4MlQrRTRFWWxKQTNuVWw3RkE0TFBmYVBLV1pNc0xMR1JCSVVMWU9FYTFiZmtTL1JvV29BMCtvN1pRRVZDYWJnSTdiRUQySldkNmhtcmptYlJzNndJcDlRVzVLOEN3SVo2VWY5RjFxMEQ1Yk5pa2xsR2grYmlSbFdTbnBMMWxwT2hXb2tYMXVCbFNFUUgzbzNibVVxUzVVWlI0ZmFMbk1ucThkdG0vMGJyY041MGhTYitMU05WZHdnWkxLM000R05sRHhnc3A5MWNFbmI2aGZLS3pnUmNFQktLTEExdW9wVzR3QkRvQWphbllqZUJWVU92QWRCOXpOOU93PT0iLCJwb2xpY3lCaW5kaW5nIjp7ImFsZyI6IkhTMjU2IiwiaGFzaCI6IlpqQXpNR1l5WXpJeFpUQm1Nek5tTWpoaE1qRmpaakkyWkROaFpUazJOREUzWkRCaFpUazNOREkyTURBMU56VTFNVFUxTVdGaU0yUmpPRFExWm1NMllnPT0ifSwia2lkIjoicjEifV0sIm1ldGhvZCI6eyJhbGdvcml0aG0iOiJBRVMtMjU2LUdDTSIsIml2IjoiIiwiaXNTdHJlYW1hYmxlIjp0cnVlfSwiaW50ZWdyaXR5SW5mb3JtYXRpb24iOnsicm9vdFNpZ25hdHVyZSI6eyJhbGciOiJIUzI1NiIsInNpZyI6IlpHVmhZbUZrTURobE1EQm1NVFZtWXpCbU1XRTBNMkpoTmpoa05qQTFaVGsxTVRkbVpqaGtaRE5rTXpJNE5XWmtNV1F4TlRWbFl6YzBNRFV4T0RNd05nPT0ifSwic2VnbWVudEhhc2hBbGciOiJHTUFDIiwic2VnbWVudFNpemVEZWZhdWx0IjoyMDk3MTUyLCJlbmNyeXB0ZWRTZWdtZW50U2l6ZURlZmF1bHQiOjIwOTcxODAsInNlZ21lbnRzIjpbeyJoYXNoIjoiTWpJM1kyRm1PVEV3TWpFeE5HUTRaREU0WXpZMFkySXlOMkkxTnpkak16az0iLCJzZWdtZW50U2l6ZSI6OCwiZW5jcnlwdGVkU2VnbWVudFNpemUiOjM2fV19fSwicGF5bG9hZCI6eyJ0eXBlIjoicmVmZXJlbmNlIiwidXJsIjoiMC5wYXlsb2FkIiwicHJvdG9jb2wiOiJ6aXAiLCJtaW1lVHlwZSI6ImFwcGxpY2F0aW9uL29jdGV0LXN0cmVhbSIsImlzRW5jcnlwdGVkIjp0cnVlfX1QSwcIMKRTpa8FAACvBQAAUEsBAhQAFAAIAAAAAAAAAGiY7skkAAAAJAAAAAkAAAAAAAAAAAAAAAAAAAAAADAucGF5bG9hZFBLAQIUABQACAAAAAAAAAAwpFOlrwUAAK8FAAANAAAAAAAAAAAAAAAAAFsAAABtYW5pZmVzdC5qc29uUEsFBgAAAAACAAIAcgAAAEUGAAAAAA=="
	badStandardTdf        = "UEsDBC0ACAAAAJ2TFTEAAAAAAAAAAAAAAAAJAAAAMC5wYXlsb2Fktu4m+vdwl0mtjhY3U5e7TG2o1s8ifK+RAhFNjRjGTLJ7V3w5UEsHCGiY7skkAAAAJAAAAFBLAwQtAAgAAACdkxUxAAAAAAAAAAAAAAAADwAAADAubWFuaWZlc3QuanNvbnsiZW5jcnlwdGlvbkluZm9ybWF0aW9uIjp7InR5cGUiOiJzcGxpdCIsInBvbGljeSI6ImV5SjFkV2xrSWpvaU1HTTFORGsyWlRZdE5EYzRaaTB4TVdWbUxXSXlOakV0WWpJMVl6UmhORE14TjJFM0lpd2lZbTlrZVNJNmV5SmtZWFJoUVhSMGNtbGlkWFJsY3lJNlczc2lZWFIwY21saWRYUmxJam9pYUhSMGNITTZMeTlsZUdGdGNHeGxMbU52YlM5aGRIUnlMMkYwZEhJeEwzWmhiSFZsTDNaaGJIVmxNU0lzSW1ScGMzQnNZWGxPWVcxbElqb2lJaXdpYVhORVpXWmhkV3gwSWpwbVlXeHpaU3dpY0hWaVMyVjVJam9pSWl3aWEyRnpWVkpNSWpvaUluMWRMQ0prYVhOelpXMGlPbHRkZlgwPSIsImtleUFjY2VzcyI6W3sidXJsIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwIiwicHJvdG9jb2wiOiJrYXMiLCJ3cmFwcGVkS2V5IjoidFVTL1BPU2lQbThlejhoci9nTFRjemNZTk9La3FDRHJWYkEwVnR2Z2tvUGxwdDNQQ2VaU3QzZ3Z5UDVWSmVwTTJjanVQYVFiSVBpcjI5VnVSdk9UV2ZkM0VIdSs4MlQrRTRFWWxKQTNuVWw3RkE0TFBmYVBLV1pNc0xMR1JCSVVMWU9FYTFiZmtTL1JvV29BMCtvN1pRRVZDYWJnSTdiRUQySldkNmhtcmptYlJzNndJcDlRVzVLOEN3SVo2VWY5RjFxMEQ1Yk5pa2xsR2grYmlSbFdTbnBMMWxwT2hXb2tYMXVCbFNFUUgzbzNibVVxUzVVWlI0ZmFMbk1ucThkdG0vMGJyY041MGhTYitMU05WZHdnWkxLM000R05sRHhnc3A5MWNFbmI2aGZLS3pnUmNFQktLTEExdW9wVzR3QkRvQWphbllqZUJWVU92QWRCOXpOOU93PT0iLCJwb2xpY3lCaW5kaW5nIjp7ImFsZyI6IkhTMjU2IiwiaGFzaCI6IlpqQXpNR1l5WXpJeFpUQm1Nek5tTWpoaE1qRmpaakkyWkROaFpUazJOREUzWkRCaFpUazNOREkyTURBMU56VTFNVFUxTVdGaU0yUmpPRFExWm1NMllnPT0ifSwia2lkIjoicjEifV0sIm1ldGhvZCI6eyJhbGdvcml0aG0iOiJBRVMtMjU2LUdDTSIsIml2IjoiIiwiaXNTdHJlYW1hYmxlIjp0cnVlfSwiaW50ZWdyaXR5SW5mb3JtYXRpb24iOnsicm9vdFNpZ25hdHVyZSI6eyJhbGciOiJIUzI1NiIsInNpZyI6IlpHVmhZbUZrTURobE1EQm1NVFZtWXpCbU1XRTBNMkpoTmpoa05qQTFaVGsxTVRkbVpqaGtaRE5rTXpJNE5XWmtNV1F4TlRWbFl6YzBNRFV4T0RNd05nPT0ifSwic2VnbWVudEhhc2hBbGciOiJHTUFDIiwic2VnbWVudFNpemVEZWZhdWx0IjoyMDk3MTUyLCJlbmNyeXB0ZWRTZWdtZW50U2l6ZURlZmF1bHQiOjIwOTcxODAsInNlZ21lbnRzIjpbeyJoYXNoIjoiTWpJM1kyRm1PVEV3TWpFeE5HUTRaREU0WXpZMFkySXlOMkkxTnpkak16az0iLCJzZWdtZW50U2l6ZSI6OCwiZW5jcnlwdGVkU2VnbWVudFNpemUiOjM2fV19fSwicGF5bG9hZCI6eyJ0eXBlIjoicmVmZXJlbmNlIiwidXJsIjoiMC5wYXlsb2FkIiwicHJvdG9jb2wiOiJ6aXAiLCJtaW1lVHlwZSI6ImFwcGxpY2F0aW9uL29jdGV0LXN0cmVhbSIsImlzRW5jcnlwdGVkIjp0cnVlfX1QSwcIMKRTpa8FAACvBQAAUEsBAi0ALQAIAAAAnZMVMWiY7skkAAAAJAAAAAkAAAAAAAAAAAAAAAAAAAAAADAucGF5bG9hZFBLAQItAC0ACAAAAJ2TFTEwpFOlrwUAAK8FAAAPAAAAAAAAAAAAAAAAAFsAAAAwLm1hbmlmZXN0Lmpzb25QSwUGAAAAAAIAAgB0AAAARwYAAAAA"
	missingPayloadTypeTdf = "UEsDBC0ACAAAAN2oTzIAAAAAAAAAAAAAAAAJAAAAMC5wYXlsb2FkD/U+BGl0DU+fwM4j8f6FgpXSaKlOvzGgSK/AnAzDUiID+S97s7fGqV7ajuc9uFBLBwgYUH8dLgAAAC4AAABQSwMELQAIAAAA3ahPMgAAAAAAAAAAAAAAAA8AAAAwLm1hbmlmZXN0Lmpzb257ImVuY3J5cHRpb25JbmZvcm1hdGlvbiI6eyJ0eXBlIjoic3BsaXQiLCJwb2xpY3kiOiJleUoxZFdsa0lqb2lPRFpqTW1ZNU16SXRaRE00TkMweE1XVm1MV0V3Tm1JdFpXRmtNR1F3TjJJeFpHUm1JaXdpWW05a2VTSTZleUprWVhSaFFYUjBjbWxpZFhSbGN5STZXM3NpWVhSMGNtbGlkWFJsSWpvaWFIUjBjSE02THk5bGVHRnRjR3hsTG1OdmJTOWhkSFJ5TDJGMGRISXhMM1poYkhWbEwzWmhiSFZsTVNJc0ltUnBjM0JzWVhsT1lXMWxJam9pSWl3aWFYTkVaV1poZFd4MElqcG1ZV3h6WlN3aWNIVmlTMlY1SWpvaUlpd2lhMkZ6VlZKTUlqb2lJbjFkTENKa2FYTnpaVzBpT2x0ZGZYMD0iLCJrZXlBY2Nlc3MiOlt7InR5cGUiOiJ3cmFwcGVkIiwidXJsIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwIiwicHJvdG9jb2wiOiJrYXMiLCJ3cmFwcGVkS2V5IjoiazR4ci9UZkpMbGpkOUM4VXR5Q3pQaG5JUkRBek9RUzA0bTZreDVlUmh1VEtmZkErNEIxajJIWjBTUDRwb2tRSGo2bDAycU1aalBkVEd4eDRnNm5wemN2QkdoeFhyQyt4Q3dCTHcrWjViTVI4VG1uOVpsOUhrUmJWUXNpb01QbWk2NURaY0RwOHZPTUxob0ZrblA4WlNsQ3ZuNHBXMWN2eEpGN3N5MGpvNDBlZDhhUVp6RnU1b0J4alpCbDhUcnpSZ1NGK3VwTGp1cEdKUHF0VVpmd0FQZ3JGTFdRMmFXQ0laQ2VkTDlCSXozTFhRS3lGU1VEUmZiTyszT3dPZHV6aWpNbVUrWnJPSGFkNXRqRklxS0swZHlRMkFDR3RoajNIUEJDUGc0UDJoZ0tGeEgzQWUrMTVFVnV5QWpGOVk4NDlDU2Q4NGJMS0NzTjZ4Mjl0L2dpVGJnPT0iLCJwb2xpY3lCaW5kaW5nIjp7ImFsZyI6IkhTMjU2IiwiaGFzaCI6Ik1UWTFZelExTjJRNU5qRTVPREprWXpjM056QTRaVFZpWlRWaE56QTNNMlpoWldNNU16azVaR1U1WmpVd1kySmhNakJsTVRBeE5HWmhaRGhrWlRFMk5RPT0ifSwia2lkIjoicjEifV0sIm1ldGhvZCI6eyJhbGdvcml0aG0iOiJBRVMtMjU2LUdDTSIsIml2IjoiIiwiaXNTdHJlYW1hYmxlIjp0cnVlfSwiaW50ZWdyaXR5SW5mb3JtYXRpb24iOnsicm9vdFNpZ25hdHVyZSI6eyJhbGciOiJIUzI1NiIsInNpZyI6Ik1ESTNaREUyTmpBek1qUXhOMkUxWlRRNU1qVTFPREF5WW1abE5UQmtaak5rTWpBeU1qa3pNbUkwTUdRd01EWTNNakkzTm1VeFptTmhPRGd4TlRVellnPT0ifSwic2VnbWVudEhhc2hBbGciOiJHTUFDIiwic2VnbWVudFNpemVEZWZhdWx0IjoyMDk3MTUyLCJlbmNyeXB0ZWRTZWdtZW50U2l6ZURlZmF1bHQiOjIwOTcxODAsInNlZ21lbnRzIjpbeyJoYXNoIjoiTlRJeU1qQXpaamt5WmpkaVlqTmlOMk0yWVRrMVpXUmhPR1ZsTnpOa1lqZz0iLCJzZWdtZW50U2l6ZSI6MTgsImVuY3J5cHRlZFNlZ21lbnRTaXplIjo0Nn1dfX0sInBheWxvYWQiOnsidXJsIjoiMC5wYXlsb2FkIiwicHJvdG9jb2wiOiJ6aXAiLCJtaW1lVHlwZSI6ImFwcGxpY2F0aW9uL29jdGV0LXN0cmVhbSIsImlzRW5jcnlwdGVkIjp0cnVlfX1QSwcI2lLjxJ0FAACdBQAAUEsBAi0ALQAIAAAA3ahPMhhQfx0uAAAALgAAAAkAAAAAAAAAAAAAAAAAAAAAADAucGF5bG9hZFBLAQItAC0ACAAAAN2oTzLaUuPEnQUAAJ0FAAAPAAAAAAAAAAAAAAAAAGUAAAAwLm1hbmlmZXN0Lmpzb25QSwUGAAAAAAIAAgB0AAAAPwYAAAAA"
)

// fakePlatform is an httptest platform: well-known and OIDC discovery, a
// token endpoint, a KAS with RSA and EC keys, the KAS registry and the
// health check, implemented with Go's crypto so the SDK is tested against
// an independent implementation of the wire format.
type fakePlatform struct {
	t              *testing.T
	server         *httptest.Server
	kasRSA         *rsa.PrivateKey
	kasEC          *ecdsa.PrivateKey
	mode           atomic.Value
	tokenType      atomic.Value
	baseKey        atomic.Value
	nonce          atomic.Value
	tokenCalls     atomic.Int32
	rewrapCalls    atomic.Int32
	publicKeyCalls atomic.Int32
	registryCalls  atomic.Int32
	mu             sync.Mutex
	lastRewrap     map[string]any
	lastDPoP       string
	lastAuth       string
}

func newFakePlatform(t *testing.T) *fakePlatform {
	t.Helper()
	f := &fakePlatform{t: t}
	f.mode.Store("")
	f.tokenType.Store("Bearer")
	f.baseKey.Store("")
	f.nonce.Store("")
	var err error
	f.kasRSA, err = rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		t.Fatal(err)
	}
	f.kasEC, err = ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatal(err)
	}
	f.server = httptest.NewServer(http.HandlerFunc(f.handle))
	t.Cleanup(f.server.Close)
	return f
}

func (f *fakePlatform) issuer() string { return f.server.URL + "/auth/realms/opentdf" }
func (f *fakePlatform) kasURL() string { return f.server.URL + "/kas" }

func publicPEM(t *testing.T, pub any) string {
	t.Helper()
	der, err := x509.MarshalPKIXPublicKey(pub)
	if err != nil {
		t.Fatal(err)
	}
	return string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der}))
}

func privatePEM(t *testing.T, key any) string {
	t.Helper()
	der, err := x509.MarshalPKCS8PrivateKey(key)
	if err != nil {
		t.Fatal(err)
	}
	return string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: der}))
}

func parsePublicPEM(data string) (any, error) {
	block, _ := pem.Decode([]byte(data))
	if block == nil {
		return nil, errors.New("no pem")
	}
	return x509.ParsePKIXPublicKey(block.Bytes)
}

func hkdf32(secret []byte) []byte {
	salt := sha256.Sum256([]byte("TDF"))
	extract := hmac.New(sha256.New, salt[:])
	extract.Write(secret)
	prk := extract.Sum(nil)
	expand := hmac.New(sha256.New, prk)
	expand.Write([]byte{1})
	return expand.Sum(nil)[:32]
}

func gcmOpen(key, frame []byte) ([]byte, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}
	if len(frame) < 12 {
		return nil, errors.New("short frame")
	}
	return gcm.Open(nil, frame[:12], frame[12:], nil)
}

func gcmSeal(key, plain []byte) []byte {
	block, _ := aes.NewCipher(key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, 12)
	rand.Read(nonce)
	return append(nonce, gcm.Seal(nil, nonce, plain, nil)...)
}

func ecdhSecret(priv *ecdsa.PrivateKey, pub *ecdsa.PublicKey) ([]byte, error) {
	a, err := priv.ECDH()
	if err != nil {
		return nil, err
	}
	b, err := pub.ECDH()
	if err != nil {
		return nil, err
	}
	return a.ECDH(b)
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func (f *fakePlatform) wellKnown() map[string]any {
	issuer := f.issuer()
	cfg := map[string]any{
		"platform_issuer": issuer,
		"idp": map[string]any{
			"issuer":                 issuer,
			"authorization_endpoint": issuer + "/protocol/openid-connect/auth",
			"token_endpoint":         issuer + "/protocol/openid-connect/token",
		},
		"health": map[string]any{"endpoint": "/healthz"},
	}
	switch f.baseKey.Load().(string) {
	case "rsa":
		cfg["base_key"] = map[string]any{"kas_uri": f.kasURL(), "public_key": map[string]any{"algorithm": "rsa:2048", "kid": "r1", "pem": publicPEM(f.t, &f.kasRSA.PublicKey)}}
	case "ec":
		cfg["base_key"] = map[string]any{"kas_uri": f.kasURL(), "public_key": map[string]any{"algorithm": "ec:secp256r1", "kid": "e1", "pem": publicPEM(f.t, &f.kasEC.PublicKey)}}
	case "invalid":
		cfg["base_key"] = "invalid-base-key"
	case "empty":
		cfg["base_key"] = map[string]any{}
	case "no-public-key":
		cfg["base_key"] = map[string]any{"kas_uri": f.kasURL()}
	case "string-public-key":
		cfg["base_key"] = map[string]any{"kas_uri": f.kasURL(), "public_key": "invalid-public-key"}
	}
	return cfg
}

func (f *fakePlatform) handle(w http.ResponseWriter, r *http.Request) {
	mode := f.mode.Load().(string)
	body, _ := io.ReadAll(r.Body)
	switch {
	case r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
		if mode == "wellknown-error" {
			writeJSON(w, 500, map[string]any{"code": "internal", "message": "service unavailable"})
			return
		}
		if mode == "wellknown-empty" {
			writeJSON(w, 200, map[string]any{"configuration": map[string]any{}})
			return
		}
		writeJSON(w, 200, map[string]any{"configuration": f.wellKnown()})
	case r.URL.Path == "/auth/realms/opentdf/.well-known/openid-configuration":
		writeJSON(w, 200, map[string]any{"issuer": f.issuer(), "token_endpoint": f.issuer() + "/protocol/openid-connect/token"})
	case r.URL.Path == "/auth/realms/opentdf/protocol/openid-connect/token":
		f.tokenCalls.Add(1)
		user, pass, ok := r.BasicAuth()
		f.mu.Lock()
		f.lastDPoP = r.Header.Get("DPoP")
		f.mu.Unlock()
		if !ok || user != "opentdf" || pass != "secret" {
			writeJSON(w, 401, map[string]any{"error": "invalid_client", "error_description": "Invalid client credentials"})
			return
		}
		if nonce := f.nonce.Load().(string); nonce != "" && !strings.Contains(f.proofClaims(r.Header.Get("DPoP")), "\"nonce\":\""+nonce+"\"") {
			w.Header().Set("DPoP-Nonce", nonce)
			writeJSON(w, 400, map[string]any{"error": "use_dpop_nonce"})
			return
		}
		writeJSON(w, 200, map[string]any{"access_token": "access-token-" + f.tokenType.Load().(string), "token_type": f.tokenType.Load().(string), "expires_in": 300})
	case strings.HasSuffix(r.URL.Path, "/kas.AccessService/PublicKey"):
		f.publicKeyCalls.Add(1)
		var req struct {
			Algorithm string `json:"algorithm"`
			V         string `json:"v"`
		}
		json.Unmarshal(body, &req)
		if req.Algorithm == "ec:secp256r1" {
			writeJSON(w, 200, map[string]any{"publicKey": publicPEM(f.t, &f.kasEC.PublicKey), "kid": "e1"})
			return
		}
		writeJSON(w, 200, map[string]any{"publicKey": publicPEM(f.t, &f.kasRSA.PublicKey), "kid": "r1"})
	case strings.HasSuffix(r.URL.Path, "/kas.AccessService/Rewrap"):
		f.rewrapCalls.Add(1)
		f.mu.Lock()
		f.lastAuth = r.Header.Get("Authorization")
		f.mu.Unlock()
		if mode == "rewrap-unauthenticated" {
			writeJSON(w, 401, map[string]any{"code": "unauthenticated", "message": "invalid token"})
			return
		}
		f.rewrap(w, body, mode)
	case r.URL.Path == "/policy.kasregistry.KeyAccessServerRegistryService/ListKeyAccessServers":
		f.registryCalls.Add(1)
		if mode == "registry-error" {
			writeJSON(w, 500, map[string]any{"code": "internal", "message": "registry down"})
			return
		}
		writeJSON(w, 200, map[string]any{"keyAccessServers": []any{map[string]any{"uri": f.kasURL(), "name": "kas"}}})
	case r.URL.Path == "/grpc.health.v1.Health/Check":
		status := "SERVING"
		if mode == "unhealthy" {
			status = "NOT_SERVING"
		}
		writeJSON(w, 200, map[string]any{"status": status})
	default:
		writeJSON(w, 404, map[string]any{"code": "unimplemented", "message": "no " + r.URL.Path})
	}
}

func (f *fakePlatform) proofClaims(proof string) string {
	parts := strings.Split(proof, ".")
	if len(parts) != 3 {
		return ""
	}
	claims, _ := base64.RawURLEncoding.DecodeString(parts[1])
	return string(claims)
}

type fakeKAO struct {
	Type               string `json:"type"`
	URL                string `json:"url"`
	WrappedKey         string `json:"wrappedKey"`
	EphemeralPublicKey string `json:"ephemeralPublicKey"`
	PolicyBinding      struct {
		Alg  string `json:"alg"`
		Hash string `json:"hash"`
	} `json:"policyBinding"`
}

func (f *fakePlatform) rewrap(w http.ResponseWriter, body []byte, mode string) {
	var req struct {
		SignedRequestToken string `json:"signedRequestToken"`
	}
	json.Unmarshal(body, &req)
	parts := strings.Split(req.SignedRequestToken, ".")
	if len(parts) != 3 {
		writeJSON(w, 400, map[string]any{"code": "invalid_argument", "message": "bad token"})
		return
	}
	claims, _ := base64.RawURLEncoding.DecodeString(parts[1])
	var c struct {
		RequestBody string `json:"requestBody"`
	}
	json.Unmarshal(claims, &c)
	var unsigned struct {
		ClientPublicKey string `json:"clientPublicKey"`
		Requests        []struct {
			Policy struct {
				ID   string `json:"id"`
				Body string `json:"body"`
			} `json:"policy"`
			KeyAccessObjects []struct {
				KeyAccessObjectID string  `json:"keyAccessObjectId"`
				KeyAccessObject   fakeKAO `json:"keyAccessObject"`
			} `json:"keyAccessObjects"`
		} `json:"requests"`
	}
	if err := json.Unmarshal([]byte(c.RequestBody), &unsigned); err != nil {
		writeJSON(w, 400, map[string]any{"code": "invalid_argument", "message": "bad request body"})
		return
	}
	var generic map[string]any
	json.Unmarshal([]byte(c.RequestBody), &generic)
	f.mu.Lock()
	f.lastRewrap = generic
	f.mu.Unlock()
	clientPub, err := parsePublicPEM(unsigned.ClientPublicKey)
	if err != nil {
		writeJSON(w, 400, map[string]any{"code": "invalid_argument", "message": "bad client key"})
		return
	}
	response := map[string]any{"responses": []any{}}
	var sessionPublicKey string
	var ephemeral *ecdsa.PrivateKey
	if ecPub, ok := clientPub.(*ecdsa.PublicKey); ok {
		ephemeral, _ = ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
		sessionPublicKey = publicPEM(f.t, &ephemeral.PublicKey)
		_ = ecPub
	}
	for _, pr := range unsigned.Requests {
		results := []any{}
		for _, kao := range pr.KeyAccessObjects {
			result := map[string]any{"keyAccessObjectId": kao.KeyAccessObjectID}
			dek, err := f.unwrap(kao.KeyAccessObject)
			if err == nil {
				expected := base64.StdEncoding.EncodeToString([]byte(hex.EncodeToString(hmacSHA256(dek, []byte(pr.Policy.Body)))))
				if expected != kao.KeyAccessObject.PolicyBinding.Hash {
					err = errors.New("rpc error: code = InvalidArgument desc = bad request")
				}
			} else {
				err = errors.New("rpc error: code = InvalidArgument desc = bad request")
			}
			if err == nil && mode == "deny" {
				err = errors.New("rpc error: code = PermissionDenied desc = forbidden")
			}
			if err == nil && mode == "misconfigured" {
				err = errors.New("rpc error: code = InvalidArgument desc = unsupported key type")
			}
			if err != nil {
				result["status"] = "fail"
				result["error"] = err.Error()
			} else {
				var wrapped []byte
				switch pub := clientPub.(type) {
				case *rsa.PublicKey:
					wrapped, _ = rsa.EncryptOAEP(sha1.New(), rand.Reader, pub, dek, nil)
				case *ecdsa.PublicKey:
					secret, _ := ecdhSecret(ephemeral, pub)
					wrapped = gcmSeal(hkdf32(secret), dek)
				}
				result["status"] = "permit"
				result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(wrapped)
				if mode == "obligations" {
					result["metadata"] = map[string]any{"X-Required-Obligations": []any{"https://example.com/obl/test/value/watermark"}}
				}
			}
			results = append(results, result)
		}
		response["responses"] = append(response["responses"].([]any), map[string]any{"policyId": pr.Policy.ID, "results": results})
	}
	if sessionPublicKey != "" {
		response["sessionPublicKey"] = sessionPublicKey
	}
	if mode == "no-policy" {
		response["responses"].([]any)[0].(map[string]any)["policyId"] = "other"
	}
	writeJSON(w, 200, response)
}

func hmacSHA256(key, data []byte) []byte {
	h := hmac.New(sha256.New, key)
	h.Write(data)
	return h.Sum(nil)
}

func (f *fakePlatform) unwrap(kao fakeKAO) ([]byte, error) {
	wrapped, err := base64.StdEncoding.DecodeString(kao.WrappedKey)
	if err != nil {
		return nil, err
	}
	switch kao.Type {
	case "wrapped":
		return rsa.DecryptOAEP(sha1.New(), rand.Reader, f.kasRSA, wrapped, nil)
	case "ec-wrapped":
		pub, err := parsePublicPEM(kao.EphemeralPublicKey)
		if err != nil {
			return nil, err
		}
		ecPub, ok := pub.(*ecdsa.PublicKey)
		if !ok {
			return nil, errors.New("not ec")
		}
		secret, err := ecdhSecret(f.kasEC, ecPub)
		if err != nil {
			return nil, err
		}
		return gcmOpen(hkdf32(secret), wrapped)
	}
	return nil, errors.New("unsupported key access type")
}

var _ = ecdh.P256

func (f *fakePlatform) options() Options {
	return Options{ClientID: "opentdf", ClientSecret: "secret"}
}

func (f *fakePlatform) newSDK(t *testing.T, opts Options) *SDK {
	t.Helper()
	s, err := New(context.Background(), f.server.URL, opts)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { s.Close() })
	return s
}

type fakeClock struct{ now int64 }

func (c *fakeClock) Now() int64 { return c.now }

func mustDecode(t *testing.T, s string) []byte {
	t.Helper()
	b, err := base64.StdEncoding.DecodeString(s)
	if err != nil {
		t.Fatal(err)
	}
	return b
}
