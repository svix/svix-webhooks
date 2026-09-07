package svix

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/url"
	"strings"

	"github.com/svix/svix-webhooks/go/internalapi"
	"github.com/svix/svix-webhooks/go/models"
)

const autoConfigTokenPrefixV1 = "auto_v1_"
const autoConfigTokenPrefixV2 = "auto_v2_"
const unsupportedTokenVersion = "Unsupported token version. You might need to update the Svix SDK to use this token"

const (
	autoConfigTokenVersionV1 = "v1"
	autoConfigTokenVersionV2 = "v2"
)

var ErrInvalidAutoConfigToken = errors.New("invalid token")

type autoConfigTokenContentV1 struct {
	AppID          string `json:"aid"`
	EndpointID     string `json:"eid"`
	ServerURL      string `json:"surl"`
	EndpointSecret string `json:"esec"`
	TokenPlaintext string `json:"tok"`
}

type autoConfigTokenContentV2 struct {
	AppID          string `json:"aid"`
	AutoconfigID   string `json:"sid"`
	ServerURL      string `json:"surl"`
	EndpointSecret string `json:"esec"`
	TokenPlaintext string `json:"tok"`
}

type decodedAutoConfigToken struct {
	version        string
	appID          string
	endpointID     string
	autoconfigID   string
	serverURL      string
	endpointSecret string
	tokenPlaintext string
}

// AutoConfig decodes an auto-configuration token and uses it with Svix APIs and webhook verification.
type AutoConfig struct {
	appID        string
	endpointID   string
	autoconfigID string
	endpoint     models.EndpointIn
	webhook      *Webhook
	svix         *internalapi.InternalSvix
}

func parseAutoConfigTokenPayload(token string, prefix string) ([]byte, error) {
	token, ok := strings.CutPrefix(token, prefix)
	if !ok {
		return nil, fmt.Errorf("%w: %s", ErrInvalidAutoConfigToken, unsupportedTokenVersion)
	}

	decoded, err := base64.StdEncoding.DecodeString(token)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}
	return decoded, nil
}

// decodeAutoConfigTokenV1 parses and validates a v1 auto-config token.
func decodeAutoConfigTokenV1(token string) (*autoConfigTokenContentV1, error) {
	decoded, err := parseAutoConfigTokenPayload(token, autoConfigTokenPrefixV1)
	if err != nil {
		return nil, err
	}

	var content autoConfigTokenContentV1
	if err := json.Unmarshal(decoded, &content); err != nil {
		return nil, ErrInvalidAutoConfigToken
	}
	return &content, nil
}

// decodeAutoConfigTokenV2 parses and validates a v2 auto-config token.
func decodeAutoConfigTokenV2(token string) (*autoConfigTokenContentV2, error) {
	decoded, err := parseAutoConfigTokenPayload(token, autoConfigTokenPrefixV2)
	if err != nil {
		return nil, err
	}

	var content autoConfigTokenContentV2
	if err := json.Unmarshal(decoded, &content); err != nil {
		return nil, ErrInvalidAutoConfigToken
	}
	return &content, nil
}

// decodeAutoConfigToken parses a v1 or v2 auto-config token.
func decodeAutoConfigToken(token string) (*decodedAutoConfigToken, error) {
	if strings.HasPrefix(token, autoConfigTokenPrefixV1) {
		content, err := decodeAutoConfigTokenV1(token)
		if err != nil {
			return nil, err
		}
		return &decodedAutoConfigToken{
			version:        autoConfigTokenVersionV1,
			appID:          content.AppID,
			endpointID:     content.EndpointID,
			serverURL:      content.ServerURL,
			endpointSecret: content.EndpointSecret,
			tokenPlaintext: content.TokenPlaintext,
		}, nil
	}
	if strings.HasPrefix(token, autoConfigTokenPrefixV2) {
		content, err := decodeAutoConfigTokenV2(token)
		if err != nil {
			return nil, err
		}
		return &decodedAutoConfigToken{
			version:        autoConfigTokenVersionV2,
			appID:          content.AppID,
			autoconfigID:   content.AutoconfigID,
			serverURL:      content.ServerURL,
			endpointSecret: content.EndpointSecret,
			tokenPlaintext: content.TokenPlaintext,
		}, nil
	}
	return nil, fmt.Errorf("%w: %s", ErrInvalidAutoConfigToken, unsupportedTokenVersion)
}

func newInternalSvixFromAutoConfig(serverURL string, token string) (*internalapi.InternalSvix, error) {
	parsed, err := url.Parse(serverURL)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	svx, err := internalapi.New(token, parsed, false, nil)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}
	return svx, nil
}

// NewAutoConfig parses an auto-config token and prepares subscribe and verify helpers.
func NewAutoConfig(token string, endpoint models.EndpointIn) (*AutoConfig, error) {
	decoded, err := decodeAutoConfigToken(token)
	if err != nil {
		return nil, err
	}

	webhook, err := NewWebhook(decoded.endpointSecret)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	svx, err := newInternalSvixFromAutoConfig(decoded.serverURL, decoded.tokenPlaintext)
	if err != nil {
		return nil, err
	}

	return &AutoConfig{
		appID:        decoded.appID,
		endpointID:   decoded.endpointID,
		autoconfigID: decoded.autoconfigID,
		endpoint:     endpoint,
		webhook:      webhook,
		svix:         svx,
	}, nil
}

// Subscribe registers or updates the endpoint via the auto-config API.
func (a *AutoConfig) Subscribe(ctx context.Context) (*models.EndpointOut, error) {
	if a.autoconfigID != "" {
		return a.svix.Endpoint().Autoconfig().Subscribe(
			ctx,
			a.appID,
			a.autoconfigID,
			a.endpoint,
		)
	}
	return a.svix.Endpoint().AutoConfigDeprecated().Update(
		ctx,
		a.appID,
		a.endpointID,
		models.SubscribeIn{Endpoint: &a.endpoint},
	)
}

// Verify validates the webhook payload using the endpoint signing secret from the token.
func (a *AutoConfig) Verify(payload []byte, headers http.Header) error {
	return a.webhook.Verify(payload, headers)
}
