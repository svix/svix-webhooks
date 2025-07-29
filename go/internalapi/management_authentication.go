// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type ManagementAuthentication struct {
	client *internal.SvixHttpClient
}

func newManagementAuthentication(client *internal.SvixHttpClient) *ManagementAuthentication {
	return &ManagementAuthentication{
		client: client,
	}
}

type ManagementAuthenticationCreateApiTokenOptions struct {
	IdempotencyKey *string
}

type ManagementAuthenticationExpireApiTokenOptions struct {
	IdempotencyKey *string
}

// Create a new API Token.
func (managementAuthentication *ManagementAuthentication) CreateApiToken(
	ctx context.Context,
	envId string,
	apiTokenIn models.ApiTokenIn,
	o *ManagementAuthenticationCreateApiTokenOptions,
) (*models.ApiTokenOut, error) {
	pathMap := map[string]string{
		"env_id": envId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.ApiTokenIn, models.ApiTokenOut](
		ctx,
		managementAuthentication.client,
		"POST",
		"/api/v1/management/authentication/{env_id}/api-token",
		pathMap,
		nil,
		headerMap,
		&apiTokenIn,
	)
}

// Expire the selected API Token.
func (managementAuthentication *ManagementAuthentication) ExpireApiToken(
	ctx context.Context,
	envId string,
	keyId string,
	apiTokenExpireIn models.ApiTokenExpireIn,
	o *ManagementAuthenticationExpireApiTokenOptions,
) error {
	pathMap := map[string]string{
		"env_id": envId,
		"key_id": keyId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[models.ApiTokenExpireIn, any](
		ctx,
		managementAuthentication.client,
		"POST",
		"/api/v1/management/authentication/{env_id}/api-token/{key_id}/expire",
		pathMap,
		nil,
		headerMap,
		&apiTokenExpireIn,
	)
	return err
}
