// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type DestinationAutoconfig struct {
	client *internal.SvixHttpClient
}

func newDestinationAutoconfig(client *internal.SvixHttpClient) DestinationAutoconfig {
	return DestinationAutoconfig{client}
}

// Create or update the destination for an AutoConfig subscription.
func (destinationAutoconfig DestinationAutoconfig) Subscribe(
	ctx context.Context,
	appId string,
	autoconfigId string,
	destinationIn models.DestinationIn,
) (*models.DestinationOut, error) {
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	return internal.ExecuteRequest[models.DestinationIn, models.DestinationOut](
		ctx,
		destinationAutoconfig.client,
		"PUT",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
		pathMap,
		nil,
		nil,
		&destinationIn,
	)
}
