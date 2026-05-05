package svix

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"strings"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

const autoConfigTokenPrefixV1 = "auto_v1_"

var ErrInvalidAutoConfigToken = errors.New("invalid auto-config token")

type autoconfigTokenContentV1 struct {
	AppID          string `json:"aid"`
	EndpointID     string `json:"eid"`
	ServerURL      string `json:"surl"`
	EndpointSecret string `json:"esec"`
	TokenPlaintext string `json:"tok"`
}

// AutoConfig completes auto-configuration for an endpoint using a token from the consumer-facing flow.
type AutoConfig struct {
	appId      string
	endpointId string
	endpoint   models.EndpointIn
	webhook    *Webhook
	client     *internal.SvixHttpClient
}

func decodeAutoconfigTokenV1(token string) (autoconfigTokenContentV1, error) {
	rest, ok := strings.CutPrefix(token, autoConfigTokenPrefixV1)
	if !ok {
		return autoconfigTokenContentV1{}, ErrInvalidAutoConfigToken
	}
	raw, err := base64.StdEncoding.DecodeString(rest)
	if err != nil {
		return autoconfigTokenContentV1{}, ErrInvalidAutoConfigToken
	}
	var content autoconfigTokenContentV1
	if err := json.Unmarshal(raw, &content); err != nil {
		return autoconfigTokenContentV1{}, ErrInvalidAutoConfigToken
	}
	return content, nil
}

// NewAutoConfig decodes an auto-config token and prepares subscribe and webhook verification.
func NewAutoConfig(token string, endpoint models.EndpointIn) (*AutoConfig, error) {
	content, err := decodeAutoconfigTokenV1(token)
	if err != nil {
		return nil, err
	}
	wh, err := NewWebhook(content.EndpointSecret)
	if err != nil {
		return nil, fmt.Errorf("%w: %w", ErrInvalidAutoConfigToken, err)
	}

	client := internal.DefaultSvixHttpClient(content.ServerURL)
	client.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", content.TokenPlaintext)
	client.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go", Version)

	return &AutoConfig{
		appId:      content.AppID,
		endpointId: content.EndpointID,
		endpoint:   endpoint,
		webhook:    wh,
		client:     &client,
	}, nil
}

// Subscribe registers or updates the endpoint using the auto-config API (same route as internalapi.EndpointAutoConfig).
func (a *AutoConfig) Subscribe(ctx context.Context) (*models.EndpointOut, error) {
	subscribeIn := models.NewSubscribeIn(a.endpoint)
	pathMap := map[string]string{
		"app_id":       a.appId,
		"endpoint_id": a.endpointId,
	}
	return internal.ExecuteRequest[models.SubscribeIn, models.EndpointOut](
		ctx,
		a.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
		pathMap,
		nil,
		nil,
		&subscribeIn,
	)
}

// Verify validates an incoming webhook payload using the endpoint signing secret from the token.
func (a *AutoConfig) Verify(payload []byte, headers http.Header) error {
	return a.webhook.Verify(payload, headers)
}
