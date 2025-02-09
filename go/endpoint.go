// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

type Endpoint struct {
	_client *SvixHttpClient
}

type EndpointListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

type EndpointCreateOptions struct {
	IdempotencyKey *string
}

type EndpointRecoverOptions struct {
	IdempotencyKey *string
}

type EndpointReplayMissingOptions struct {
	IdempotencyKey *string
}

type EndpointRotateSecretOptions struct {
	IdempotencyKey *string
}

type EndpointSendExampleOptions struct {
	IdempotencyKey *string
}

type EndpointGetStatsOptions struct {
	// Filter the range to data starting from this date.
	Since *time.Time
	// Filter the range to data ending by this date.
	Until *time.Time
}

// List the application's endpoints.
func (endpoint *Endpoint) List(
	ctx context.Context,
	appId string,
	o *EndpointListOptions,
) (*ListResponseEndpointOut, error) {
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
	ret, apiErr := executeRequest[ListResponseEndpointOut](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint",
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

// Create a new endpoint for the application.
//
// When `secret` is `null` the secret is automatically generated (recommended).
func (endpoint *Endpoint) Create(
	ctx context.Context,
	appId string,
	endpointIn *EndpointIn,
	o *EndpointCreateOptions,
) (*EndpointOut, error) {
	if endpointIn == nil {
		return nil, fmt.Errorf("Endpoint.Create(), endpointIn must not be nil")
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
	jsonBody, err := json.Marshal(endpointIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[EndpointOut](
		ctx,
		endpoint._client,
		"POST",
		"/api/v1/app/{app_id}/endpoint",
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

// Get an endpoint.
func (endpoint *Endpoint) Get(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[EndpointOut](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
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

// Update an endpoint.
func (endpoint *Endpoint) Update(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointUpdate *EndpointUpdate,
) (*EndpointOut, error) {
	if endpointUpdate == nil {
		return nil, fmt.Errorf("Endpoint.Update(), endpointUpdate must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(endpointUpdate)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[EndpointOut](
		ctx,
		endpoint._client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
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

// Delete an endpoint.
func (endpoint *Endpoint) Delete(
	ctx context.Context,
	appId string,
	endpointId string,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	_, apiErr := executeRequest[any](
		ctx,
		endpoint._client,
		"DELETE",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
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

// Partially update an endpoint.
func (endpoint *Endpoint) Patch(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointPatch *EndpointPatch,
) (*EndpointOut, error) {
	if endpointPatch == nil {
		return nil, fmt.Errorf("Endpoint.Patch(), endpointPatch must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(endpointPatch)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[EndpointOut](
		ctx,
		endpoint._client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
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

// Get the additional headers to be sent with the webhook.
func (endpoint *Endpoint) GetHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointHeadersOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[EndpointHeadersOut](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
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

// Set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) UpdateHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersIn *EndpointHeadersIn,
) error {
	if endpointHeadersIn == nil {
		return fmt.Errorf("Endpoint.UpdateHeaders(), endpointHeadersIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(endpointHeadersIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		endpoint._client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
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

// Partially set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) PatchHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersPatchIn *EndpointHeadersPatchIn,
) error {
	if endpointHeadersPatchIn == nil {
		return fmt.Errorf("Endpoint.PatchHeaders(), endpointHeadersPatchIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(endpointHeadersPatchIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		endpoint._client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
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

// Resend all failed messages since a given time.
//
// Messages that were sent successfully, even if failed initially, are not resent.
func (endpoint *Endpoint) Recover(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn *RecoverIn,
	o *EndpointRecoverOptions,
) (*RecoverOut, error) {
	if recoverIn == nil {
		return nil, fmt.Errorf("Endpoint.Recover(), recoverIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
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
	jsonBody, err := json.Marshal(recoverIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[RecoverOut](
		ctx,
		endpoint._client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
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

// Replays messages to the endpoint.
//
// Only messages that were created after `since` will be sent.
// Messages that were previously sent to the endpoint are not resent.
func (endpoint *Endpoint) ReplayMissing(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn *ReplayIn,
	o *EndpointReplayMissingOptions,
) (*ReplayOut, error) {
	if replayIn == nil {
		return nil, fmt.Errorf("Endpoint.ReplayMissing(), replayIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
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
	jsonBody, err := json.Marshal(replayIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[ReplayOut](
		ctx,
		endpoint._client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
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

// Get the endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (endpoint *Endpoint) GetSecret(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointSecretOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[EndpointSecretOut](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
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

// Rotates the endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (endpoint *Endpoint) RotateSecret(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn *EndpointSecretRotateIn,
	o *EndpointRotateSecretOptions,
) error {
	if endpointSecretRotateIn == nil {
		return fmt.Errorf("Endpoint.RotateSecret(), endpointSecretRotateIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
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
	jsonBody, err := json.Marshal(endpointSecretRotateIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		endpoint._client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
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

// Send an example message for an event.
func (endpoint *Endpoint) SendExample(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
	o *EndpointSendExampleOptions,
) (*MessageOut, error) {
	if eventExampleIn == nil {
		return nil, fmt.Errorf("Endpoint.SendExample(), eventExampleIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
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
	jsonBody, err := json.Marshal(eventExampleIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[MessageOut](
		ctx,
		endpoint._client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
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

// Get basic statistics for the endpoint.
func (endpoint *Endpoint) GetStats(
	ctx context.Context,
	appId string,
	endpointId string,
	o *EndpointGetStatsOptions,
) (*EndpointStats, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("since", o.Since, queryMap, &err)
		SerializeParamToMap("until", o.Until, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[EndpointStats](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
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

// Get the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationGet(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointTransformationOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[EndpointTransformationOut](
		ctx,
		endpoint._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
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

// Set or unset the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationPartialUpdate(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointTransformationIn *EndpointTransformationIn,
) error {
	if endpointTransformationIn == nil {
		return fmt.Errorf("Endpoint.TransformationPartialUpdate(), endpointTransformationIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(endpointTransformationIn)
	if err != nil {
		return err
	}
	_, apiErr := executeRequest[any](
		ctx,
		endpoint._client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
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
