package svix

import (
	"encoding/base64"
	"errors"
	"strings"
	"testing"

	"github.com/svix/svix-webhooks/go/models"
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
	if !strings.Contains(err.Error(), unsupportedTokenVersion) {
		t.Fatalf("expected unsupported token version detail, got %v", err)
	}
}

func TestDecodeAutoConfigTokenV1RejectsInvalidJSON(t *testing.T) {
	token := autoConfigTokenPrefixV1 + base64.StdEncoding.EncodeToString([]byte("not json"))

	_, err := decodeAutoConfigTokenV1(token)
	if !errors.Is(err, ErrInvalidAutoConfigToken) {
		t.Fatalf("expected ErrInvalidAutoConfigToken, got %v", err)
	}
}

func TestDecodeAutoConfigTokenV2ParsesPayload(t *testing.T) {
	jsonStr := `{"aid":"app_1","sid":"acfg_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}`
	token := autoConfigTokenPrefixV2 + base64.StdEncoding.EncodeToString([]byte(jsonStr))

	content, err := decodeAutoConfigTokenV2(token)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}
	if content.AppID != "app_1" {
		t.Fatalf("app id: got %q", content.AppID)
	}
	if content.AutoconfigID != "acfg_2" {
		t.Fatalf("autoconfig id: got %q", content.AutoconfigID)
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

func TestDecodeAutoConfigTokenV2RejectsV1Prefix(t *testing.T) {
	jsonStr := `{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}`
	token := autoConfigTokenPrefixV1 + base64.StdEncoding.EncodeToString([]byte(jsonStr))

	_, err := decodeAutoConfigTokenV2(token)
	if !errors.Is(err, ErrInvalidAutoConfigToken) {
		t.Fatalf("expected ErrInvalidAutoConfigToken, got %v", err)
	}
	if !strings.Contains(err.Error(), unsupportedTokenVersion) {
		t.Fatalf("expected unsupported token version detail, got %v", err)
	}
}

func TestDecodeAutoConfigToken(t *testing.T) {
	v1JSON := `{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}`
	v2JSON := `{"aid":"app_1","sid":"acfg_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}`

	tests := []struct {
		name           string
		token          string
		wantVersion    string
		wantAppID      string
		wantEndpointID string
		wantAutoCfgID  string
		wantErr        bool
		wantErrMsg     string
	}{
		{
			name:           "v1",
			token:          autoConfigTokenPrefixV1 + base64.StdEncoding.EncodeToString([]byte(v1JSON)),
			wantVersion:    autoConfigTokenVersionV1,
			wantAppID:      "app_1",
			wantEndpointID: "ep_2",
		},
		{
			name:          "v2",
			token:         autoConfigTokenPrefixV2 + base64.StdEncoding.EncodeToString([]byte(v2JSON)),
			wantVersion:   autoConfigTokenVersionV2,
			wantAppID:     "app_1",
			wantAutoCfgID: "acfg_2",
		},
		{
			name:       "unknown prefix",
			token:      "wrong_" + base64.StdEncoding.EncodeToString([]byte(v1JSON)),
			wantErr:    true,
			wantErrMsg: unsupportedTokenVersion,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			decoded, err := decodeAutoConfigToken(tt.token)
			if tt.wantErr {
				if !errors.Is(err, ErrInvalidAutoConfigToken) {
					t.Fatalf("expected ErrInvalidAutoConfigToken, got %v", err)
				}
				if tt.wantErrMsg != "" && !strings.Contains(err.Error(), tt.wantErrMsg) {
					t.Fatalf("expected error to contain %q, got %v", tt.wantErrMsg, err)
				}
				return
			}
			if err != nil {
				t.Fatalf("decode: %v", err)
			}
			if decoded.version != tt.wantVersion {
				t.Fatalf("version: got %q want %q", decoded.version, tt.wantVersion)
			}
			if decoded.appID != tt.wantAppID {
				t.Fatalf("app id: got %q want %q", decoded.appID, tt.wantAppID)
			}
			if decoded.endpointID != tt.wantEndpointID {
				t.Fatalf("endpoint id: got %q want %q", decoded.endpointID, tt.wantEndpointID)
			}
			if decoded.autoconfigID != tt.wantAutoCfgID {
				t.Fatalf("autoconfig id: got %q want %q", decoded.autoconfigID, tt.wantAutoCfgID)
			}
		})
	}
}

func TestSinkInCommonToPollingDestination(t *testing.T) {
	uid := "uid_1"
	metadata := map[string]string{"k": "v"}
	sink := models.SinkInCommon{
		Uid:        &uid,
		EventTypes: []string{"issue.opened"},
		Channels:   []string{"ch1"},
		Metadata:   &metadata,
	}

	got := sinkInCommonToPollingDestination(sink)
	if got.Type != models.DestinationInTypePollingEndpoint {
		t.Fatalf("type: got %q", got.Type)
	}
	if got.Uid == nil || *got.Uid != uid {
		t.Fatalf("uid: got %v", got.Uid)
	}
	if len(got.EventTypes) != 1 || got.EventTypes[0] != "issue.opened" {
		t.Fatalf("event types: got %v", got.EventTypes)
	}
	if len(got.Channels) != 1 || got.Channels[0] != "ch1" {
		t.Fatalf("channels: got %v", got.Channels)
	}
	if got.Metadata == nil || (*got.Metadata)["k"] != "v" {
		t.Fatalf("metadata: got %v", got.Metadata)
	}
	if got.Status != nil {
		t.Fatalf("status: got %v want nil", got.Status)
	}
}
