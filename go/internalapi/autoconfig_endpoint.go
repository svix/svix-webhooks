// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type AutoconfigEndpoint struct {
	client *internal.SvixHttpClient
}

func newAutoconfigEndpoint(client *internal.SvixHttpClient) AutoconfigEndpoint {
	return AutoconfigEndpoint{client}
}

// Create or update the HTTP endpoint for an AutoConfig subscription.
func (autoconfigEndpoint AutoconfigEndpoint) Subscribe(
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
		autoconfigEndpoint.client,
		"PUT",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
		pathMap,
		nil,
		nil,
		&endpointIn,
	)
}
