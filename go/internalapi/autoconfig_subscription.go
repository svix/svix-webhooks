// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type AutoconfigSubscription struct {
	client *internal.SvixHttpClient
}

func newAutoconfigSubscription(client *internal.SvixHttpClient) AutoconfigSubscription {
	return AutoconfigSubscription{client}
}

func (autoconfigSubscription AutoconfigSubscription) Destination() AutoconfigSubscriptionDestination {
	return newAutoconfigSubscriptionDestination(autoconfigSubscription.client)
}
func (autoconfigSubscription AutoconfigSubscription) Endpoint() AutoconfigSubscriptionEndpoint {
	return newAutoconfigSubscriptionEndpoint(autoconfigSubscription.client)
}

// Get an AutoConfig subscription, including the bound endpoint or destination if any.
func (autoconfigSubscription AutoconfigSubscription) Get(
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
		autoconfigSubscription.client,
		"GET",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}
