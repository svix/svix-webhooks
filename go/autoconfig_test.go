package svix

import (
	"encoding/base64"
	"errors"
	"testing"
)

func TestDecodeAutoConfigTokenV1ParsesPayload(t *testing.T) {
	jsonStr := `{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}`
	token := autoConfigTokenPrefixV1 + base64.StdEncoding.EncodeToString([]byte(jsonStr))

	content, err := decodeAutoConfigTokenV1(token)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}
	if content.AppID != "app_1" {
		t.Fatalf("app id: got %q", content.AppID)
	}
	if content.EndpointID != "ep_2" {
		t.Fatalf("endpoint id: got %q", content.EndpointID)
	}
	if content.ServerURL != "https://api.example.test" {
		t.Fatalf("server url: got %q", content.ServerURL)
	}
	if content.EndpointSecret != "whsec_Zm9v" {
		t.Fatalf("endpoint secret: got %q", content.EndpointSecret)
	}
	if content.TokenPlaintext != "sk_test_xyz" {
		t.Fatalf("token: got %q", content.TokenPlaintext)
	}
}

func TestDecodeAutoConfigTokenV1RejectsBadPrefix(t *testing.T) {
	jsonStr := `{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}`
	token := "wrong_" + base64.StdEncoding.EncodeToString([]byte(jsonStr))

	_, err := decodeAutoConfigTokenV1(token)
	if !errors.Is(err, ErrInvalidAutoConfigToken) {
		t.Fatalf("expected ErrInvalidAutoConfigToken, got %v", err)
	}
}

func TestDecodeAutoConfigTokenV1RejectsInvalidJSON(t *testing.T) {
	token := autoConfigTokenPrefixV1 + base64.StdEncoding.EncodeToString([]byte("not json"))

	_, err := decodeAutoConfigTokenV1(token)
	if !errors.Is(err, ErrInvalidAutoConfigToken) {
		t.Fatalf("expected ErrInvalidAutoConfigToken, got %v", err)
	}
}
