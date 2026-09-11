// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Autoconfig struct {
	client *internal.SvixHttpClient
}

func newAutoconfig(client *internal.SvixHttpClient) Autoconfig {
	return Autoconfig{client}
}

func (autoconfig Autoconfig) Destination() AutoconfigDestination {
	return newAutoconfigDestination(autoconfig.client)
}
func (autoconfig Autoconfig) Endpoint() AutoconfigEndpoint {
	return newAutoconfigEndpoint(autoconfig.client)
}

// Get an AutoConfig subscription, including the bound endpoint or destination if any.
func (autoconfig Autoconfig) Get(
	ctx context.Context,
	appId string,
	autoconfigId string,
) (*models.AutoConfigSubscriptionOut, error) {
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	return internal.ExecuteRequest[any, models.AutoConfigSubscriptionOut](
		ctx,
		autoconfig.client,
		"GET",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}
