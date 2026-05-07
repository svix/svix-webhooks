package svix

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"net/http"
	"net/url"
	"strings"

	"github.com/svix/svix-webhooks/go/internalapi"
	"github.com/svix/svix-webhooks/go/models"
)

const autoConfigTokenPrefixV1 = "auto_v1_"

var ErrInvalidAutoConfigToken = errors.New("invalid token")

type autoConfigTokenContentV1 struct {
	AppID          string `json:"aid"`
	EndpointID     string `json:"eid"`
	ServerURL      string `json:"surl"`
	EndpointSecret string `json:"esec"`
	TokenPlaintext string `json:"tok"`
}

// AutoConfig decodes an auto-configuration token and uses it with Svix APIs and webhook verification.
type AutoConfig struct {
	appID      string
	endpointID string
	endpoint   models.EndpointIn
	webhook    *Webhook
	svix       *internalapi.InternalSvix
}

func decodeAutoConfigTokenV1(token string) (*autoConfigTokenContentV1, error) {
	token, ok := strings.CutPrefix(token, autoConfigTokenPrefixV1)
	if !ok {
		return nil, ErrInvalidAutoConfigToken
	}

	decoded, err := base64.StdEncoding.DecodeString(token)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	var content autoConfigTokenContentV1
	if err := json.Unmarshal(decoded, &content); err != nil {
		return nil, ErrInvalidAutoConfigToken
	}
	return &content, nil
}

// NewAutoConfig parses a v1 auto-config token and prepares subscribe and verify helpers.
func NewAutoConfig(token string, endpoint models.EndpointIn) (*AutoConfig, error) {
	content, err := decodeAutoConfigTokenV1(token)
	if err != nil {
		return nil, err
	}

	webhook, err := NewWebhook(content.EndpointSecret)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	serverURL, err := url.Parse(content.ServerURL)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	svx, err := internalapi.New(content.TokenPlaintext, serverURL, false, nil)
	if err != nil {
		return nil, ErrInvalidAutoConfigToken
	}

	return &AutoConfig{
		appID:      content.AppID,
		endpointID: content.EndpointID,
		endpoint:   endpoint,
		webhook:    webhook,
		svix:       svx,
	}, nil
}

// Subscribe registers or updates the endpoint via the auto-config API.
func (a *AutoConfig) Subscribe(ctx context.Context) (*models.EndpointOut, error) {
	return a.svix.Endpoint.AutoConfig.Update(
		ctx,
		a.appID,
		a.endpointID,
		models.SubscribeIn{Endpoint: a.endpoint},
	)
}

// Verify validates the webhook payload using the endpoint signing secret from the token.
func (a *AutoConfig) Verify(payload []byte, headers http.Header) error {
	return a.webhook.Verify(payload, headers)
}
