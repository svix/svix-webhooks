// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type Integration struct {
	client *SvixHttpClient
}

type IntegrationListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type IntegrationCreateOptions struct {
	IdempotencyKey *string
}

type IntegrationRotateKeyOptions struct {
	IdempotencyKey *string
}

// List the application's integrations.
func (integration *Integration) List(
	ctx context.Context,
	appId string,
	o *IntegrationListOptions,
) (*models.ListResponseIntegrationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[any, models.ListResponseIntegrationOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		queryMap,
		headerMap,
		nil,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Create an integration.
func (integration *Integration) Create(
	ctx context.Context,
	appId string,
	integrationIn models.IntegrationIn,
	o *IntegrationCreateOptions,
) (*models.IntegrationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[models.IntegrationIn, models.IntegrationOut](
		ctx,
		integration.client,
		"POST",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		queryMap,
		headerMap,
		&integrationIn,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Get an integration.
func (integration *Integration) Get(
	ctx context.Context,
	appId string,
	integId string,
) (*models.IntegrationOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	ret, err := executeRequest[any, models.IntegrationOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		nil,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Update an integration.
func (integration *Integration) Update(
	ctx context.Context,
	appId string,
	integId string,
	integrationUpdate models.IntegrationUpdate,
) (*models.IntegrationOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	ret, err := executeRequest[models.IntegrationUpdate, models.IntegrationOut](
		ctx,
		integration.client,
		"PUT",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		&integrationUpdate,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Delete an integration.
func (integration *Integration) Delete(
	ctx context.Context,
	appId string,
	integId string,
) error {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	_, err := executeRequest[any, any](
		ctx,
		integration.client,
		"DELETE",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		nil,
	)
	if err != nil {
		return err
	}
	return nil
}

// Get an integration's key.
//
// Deprecated: GetKey is deprecated.
func (integration *Integration) GetKey(
	ctx context.Context,
	appId string,
	integId string,
) (*models.IntegrationKeyOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	ret, err := executeRequest[any, models.IntegrationKeyOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}/key",
		pathMap,
		queryMap,
		headerMap,
		nil,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Rotate the integration's key. The previous key will be immediately revoked.
func (integration *Integration) RotateKey(
	ctx context.Context,
	appId string,
	integId string,
	o *IntegrationRotateKeyOptions,
) (*models.IntegrationKeyOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[any, models.IntegrationKeyOut](
		ctx,
		integration.client,
		"POST",
		"/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
		pathMap,
		queryMap,
		headerMap,
		nil,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}
