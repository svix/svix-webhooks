// Package svix this file is @generated DO NOT EDIT
package svix

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

type ManagementAuthenticationListApiTokensOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type ManagementAuthenticationCreateApiTokenOptions struct {
	IdempotencyKey *string
}

type ManagementAuthenticationExpireApiTokenOptions struct {
	IdempotencyKey *string
}

// List all API Tokens.
func (managementAuthentication *ManagementAuthentication) ListApiTokens(
	ctx context.Context,
	o *ManagementAuthenticationListApiTokensOptions,
) (*models.ListResponseApiTokenCensoredOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseApiTokenCensoredOut](
		ctx,
		managementAuthentication.client,
		"GET",
		"/api/v1/management/authentication/api-token",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create a new API Token.
func (managementAuthentication *ManagementAuthentication) CreateApiToken(
	ctx context.Context,
	apiTokenIn models.ApiTokenIn,
	o *ManagementAuthenticationCreateApiTokenOptions,
) (*models.ApiTokenOut, error) {
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
		"/api/v1/management/authentication/api-token",
		nil,
		nil,
		headerMap,
		&apiTokenIn,
	)
}

// Expire the selected API Token.
func (managementAuthentication *ManagementAuthentication) ExpireApiToken(
	ctx context.Context,
	keyId string,
	apiTokenExpireIn models.ApiTokenExpireIn,
	o *ManagementAuthenticationExpireApiTokenOptions,
) error {
	pathMap := map[string]string{
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
		"/api/v1/management/authentication/api-token/{key_id}/expire",
		pathMap,
		nil,
		headerMap,
		&apiTokenExpireIn,
	)
	return err
}
