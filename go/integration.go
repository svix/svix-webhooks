// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/models"
)

type Integration struct {
	_client *SvixHttpClient
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
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseIntegrationOut](
		ctx,
		integration._client,
		"GET",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Create an integration.
func (integration *Integration) Create(
	ctx context.Context,
	appId string,
	integrationIn *models.IntegrationIn,
	o *IntegrationCreateOptions,
) (*models.IntegrationOut, error) {
	if integrationIn == nil {
		return nil, fmt.Errorf("Integration.Create(), integrationIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err := json.Marshal(integrationIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.IntegrationOut](
		ctx,
		integration._client,
		"POST",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
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
	var jsonBody []byte

	ret, apiErr := executeRequest[models.IntegrationOut](
		ctx,
		integration._client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Update an integration.
func (integration *Integration) Update(
	ctx context.Context,
	appId string,
	integId string,
	integrationUpdate *models.IntegrationUpdate,
) (*models.IntegrationOut, error) {
	if integrationUpdate == nil {
		return nil, fmt.Errorf("Integration.Update(), integrationUpdate must not be nil")
	}
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(integrationUpdate)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.IntegrationOut](
		ctx,
		integration._client,
		"PUT",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
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
	var jsonBody []byte

	_, apiErr := executeRequest[any](
		ctx,
		integration._client,
		"DELETE",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return apiErr
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
	var jsonBody []byte

	ret, apiErr := executeRequest[models.IntegrationKeyOut](
		ctx,
		integration._client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}/key",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
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
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.IntegrationKeyOut](
		ctx,
		integration._client,
		"POST",
		"/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}
