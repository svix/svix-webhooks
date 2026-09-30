// Package svix this file is @generated DO NOT EDIT
package svix

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

type AutoconfigSubscriptionCreateOptions struct {
	IdempotencyKey *string
}

type AutoconfigSubscriptionRotateOptions struct {
	IdempotencyKey *string
}

// Create an AutoConfig subscription.
func (autoconfigSubscription AutoconfigSubscription) Create(
	ctx context.Context,
	appId string,
	createAutoConfigSubscriptionIn models.CreateAutoConfigSubscriptionIn,
	o *AutoconfigSubscriptionCreateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := AutoconfigSubscriptionCreateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.CreateAutoConfigSubscriptionIn, models.AutoConfigOut](
		ctx,
		autoconfigSubscription.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig",
		pathMap,
		nil,
		headerMap,
		&createAutoConfigSubscriptionIn,
	)
}

// Delete an AutoConfig subscription. This also invalidates its auth token.
func (autoconfigSubscription AutoconfigSubscription) Delete(
	ctx context.Context,
	appId string,
	autoconfigId string,
) error {
	var err error
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		autoconfigSubscription.client,
		"DELETE",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Rotate the auth token and signing secret for an AutoConfig subscription.
func (autoconfigSubscription AutoconfigSubscription) Rotate(
	ctx context.Context,
	appId string,
	autoconfigId string,
	rotateSubscriptionIn2 models.RotateSubscriptionIn2,
	o *AutoconfigSubscriptionRotateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := AutoconfigSubscriptionRotateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.RotateSubscriptionIn2, models.AutoConfigOut](
		ctx,
		autoconfigSubscription.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
		pathMap,
		nil,
		headerMap,
		&rotateSubscriptionIn2,
	)
}
