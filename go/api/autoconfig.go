// Package svix this file is @generated DO NOT EDIT
package api

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Autoconfig struct {
	client *internal.SvixHttpClient
}

func NewAutoconfig(client *internal.SvixHttpClient) Autoconfig {
	return Autoconfig{client}
}

type AutoconfigCreateOptions struct {
	IdempotencyKey *string
}

type AutoconfigRotateOptions struct {
	IdempotencyKey *string
}

// Create an AutoConfig subscription.
func (autoconfig Autoconfig) Create(
	ctx context.Context,
	appId string,
	o *AutoconfigCreateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := AutoconfigCreateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[any, models.AutoConfigOut](
		ctx,
		autoconfig.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig",
		pathMap,
		nil,
		headerMap,
		nil,
	)
}

// Rotate the auth token and signing secret for an AutoConfig subscription.
func (autoconfig Autoconfig) Rotate(
	ctx context.Context,
	appId string,
	autoconfigId string,
	rotateSubscriptionIn2 models.RotateSubscriptionIn2,
	o *AutoconfigRotateOptions,
) (*models.AutoConfigOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id":        appId,
		"autoconfig_id": autoconfigId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := AutoconfigRotateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.RotateSubscriptionIn2, models.AutoConfigOut](
		ctx,
		autoconfig.client,
		"POST",
		"/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
		pathMap,
		nil,
		headerMap,
		&rotateSubscriptionIn2,
	)
}
