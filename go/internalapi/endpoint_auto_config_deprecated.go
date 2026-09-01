// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type EndpointAutoConfigDeprecated struct {
	client *internal.SvixHttpClient
}

func newEndpointAutoConfigDeprecated(client *internal.SvixHttpClient) EndpointAutoConfigDeprecated {
	return EndpointAutoConfigDeprecated{client}
}

// Update an auto-config endpoint by providing endpoint details.
func (endpointAutoConfigDeprecated EndpointAutoConfigDeprecated) Update(
	ctx context.Context,
	appId string,
	endpointId string,
	subscribeIn models.SubscribeIn,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[models.SubscribeIn, models.EndpointOut](
		ctx,
		endpointAutoConfigDeprecated.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
		pathMap,
		nil,
		nil,
		&subscribeIn,
	)
}
