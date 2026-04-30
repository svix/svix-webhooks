// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type EndpointMagicSubscription struct {
	client *internal.SvixHttpClient
}

func newEndpointMagicSubscription(client *internal.SvixHttpClient) *EndpointMagicSubscription {
	return &EndpointMagicSubscription{
		client: client,
	}
}

// Update a magic subscription by providing endpoint details.
func (endpointMagicSubscription *EndpointMagicSubscription) Update(
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
		endpointMagicSubscription.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/magic-subscription",
		pathMap,
		nil,
		nil,
		&subscribeIn,
	)
}
