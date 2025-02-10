// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/models"
)

type OperationalWebhookEndpoint struct {
	_client *SvixHttpClient
}

type OperationalWebhookEndpointListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type OperationalWebhookEndpointCreateOptions struct {
	IdempotencyKey *string
}

type OperationalWebhookEndpointRotateSecretOptions struct {
	IdempotencyKey *string
}

// List operational webhook endpoints.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) List(
	ctx context.Context,
	o *OperationalWebhookEndpointListOptions,
) (*models.ListResponseOperationalWebhookEndpointOut, error) {
	pathMap := map[string]string{}
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
	ret, apiErr := executeRequest[models.ListResponseOperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint._client,
		"GET",
		"/api/v1/operational-webhook/endpoint",
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

// Create an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Create(
	ctx context.Context,
	operationalWebhookEndpointIn *models.OperationalWebhookEndpointIn,
	o *OperationalWebhookEndpointCreateOptions,
) (*models.OperationalWebhookEndpointOut, error) {
	if operationalWebhookEndpointIn == nil {
		return nil, fmt.Errorf("OperationalWebhookEndpoint.Create(), operationalWebhookEndpointIn must not be nil")
	}
	pathMap := map[string]string{}
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
	jsonBody, err := json.Marshal(operationalWebhookEndpointIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint._client,
		"POST",
		"/api/v1/operational-webhook/endpoint",
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

// Get an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Get(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint._client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
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

// Update an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Update(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointUpdate *models.OperationalWebhookEndpointUpdate,
) (*models.OperationalWebhookEndpointOut, error) {
	if operationalWebhookEndpointUpdate == nil {
		return nil, fmt.Errorf("OperationalWebhookEndpoint.Update(), operationalWebhookEndpointUpdate must not be nil")
	}
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(operationalWebhookEndpointUpdate)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint._client,
		"PUT",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
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

// Delete an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Delete(
	ctx context.Context,
	endpointId string,
) error {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	_, apiErr := executeRequest[any](
		ctx,
		operationalWebhookEndpoint._client,
		"DELETE",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
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

// Get the additional headers to be sent with the operational webhook.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) GetHeaders(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointHeadersOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[models.OperationalWebhookEndpointHeadersOut](
		ctx,
		operationalWebhookEndpoint._client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
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

// Set the additional headers to be sent with the operational webhook.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) UpdateHeaders(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointHeadersIn *models.OperationalWebhookEndpointHeadersIn,
) error {
	if operationalWebhookEndpointHeadersIn == nil {
		return fmt.Errorf("OperationalWebhookEndpoint.UpdateHeaders(), operationalWebhookEndpointHeadersIn must not be nil")
	}
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(operationalWebhookEndpointHeadersIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		operationalWebhookEndpoint._client,
		"PUT",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
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

// Get an operational webhook endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) GetSecret(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointSecretOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[models.OperationalWebhookEndpointSecretOut](
		ctx,
		operationalWebhookEndpoint._client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
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

// Rotates an operational webhook endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) RotateSecret(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointSecretIn *models.OperationalWebhookEndpointSecretIn,
	o *OperationalWebhookEndpointRotateSecretOptions,
) error {
	if operationalWebhookEndpointSecretIn == nil {
		return fmt.Errorf("OperationalWebhookEndpoint.RotateSecret(), operationalWebhookEndpointSecretIn must not be nil")
	}
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	jsonBody, err := json.Marshal(operationalWebhookEndpointSecretIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		operationalWebhookEndpoint._client,
		"POST",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
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
