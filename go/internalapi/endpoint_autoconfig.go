// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type EndpointAutoconfig struct {
	client *internal.SvixHttpClient
}

func newEndpointAutoconfig(client *internal.SvixHttpClient) EndpointAutoconfig {
	return EndpointAutoconfig{client}
}

// Create or update the HTTP endpoint for an AutoConfig subscription.
func (endpointAutoconfig EndpointAutoconfig) Subscribe(
	ctx context.Context,
	appId string,
	autoconfigId string,
	endpointIn models.EndpointIn,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	return internal.ExecuteRequest[models.EndpointIn, models.EndpointOut](
		ctx,
		endpointAutoconfig.client,
		"PUT",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
		pathMap,
		nil,
		nil,
		&endpointIn,
	)
}
