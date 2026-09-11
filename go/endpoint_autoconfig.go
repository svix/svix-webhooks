// Package svix this file is @generated DO NOT EDIT
package svix

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

type EndpointAutoconfigCreateOptions struct {
	IdempotencyKey *string
}

type EndpointAutoconfigRotateOptions struct {
	IdempotencyKey *string
}

// Create an AutoConfig subscription.
func (endpointAutoconfig EndpointAutoconfig) Create(
	ctx context.Context,
	appId string,
	o *EndpointAutoconfigCreateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := EndpointAutoconfigCreateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[any, models.AutoConfigOut](
		ctx,
		endpointAutoconfig.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig",
		pathMap,
		nil,
		headerMap,
		nil,
	)
}

// Rotate the auth token and signing secret for an AutoConfig subscription.
func (endpointAutoconfig EndpointAutoconfig) Rotate(
	ctx context.Context,
	appId string,
	autoconfigId string,
	rotateSubscriptionIn2 models.RotateSubscriptionIn2,
	o *EndpointAutoconfigRotateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := EndpointAutoconfigRotateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.RotateSubscriptionIn2, models.AutoConfigOut](
		ctx,
		endpointAutoconfig.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
		pathMap,
		nil,
		headerMap,
		&rotateSubscriptionIn2,
	)
}
