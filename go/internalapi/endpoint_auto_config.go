package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

// EndpointAutoConfig calls the non-public auto-config endpoint (operation id v1.endpoint.auto-config.update).
type EndpointAutoConfig struct {
	client *internal.SvixHttpClient
}

// NewEndpointAutoConfig constructs a client for auto-config updates using the given HTTP client configuration.
func NewEndpointAutoConfig(client *internal.SvixHttpClient) *EndpointAutoConfig {
	return &EndpointAutoConfig{client: client}
}

// Update an auto-config endpoint by providing endpoint details.
func (e *EndpointAutoConfig) Update(
	ctx context.Context,
	appId string,
	endpointId string,
	subscribeIn models.SubscribeIn,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":       appId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[models.SubscribeIn, models.EndpointOut](
		ctx,
		e.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
		pathMap,
		nil,
		nil,
		&subscribeIn,
	)
}
