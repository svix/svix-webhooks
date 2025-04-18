// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type IngestEndpoint struct {
	client *internal.SvixHttpClient
}

func newIngestEndpoint(client *internal.SvixHttpClient) *IngestEndpoint {
	return &IngestEndpoint{
		client: client,
	}
}

type IngestEndpointListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type IngestEndpointCreateOptions struct {
	IdempotencyKey *string
}

type IngestEndpointRotateSecretOptions struct {
	IdempotencyKey *string
}

// List ingest endpoints.
func (ingestEndpoint *IngestEndpoint) List(
	ctx context.Context,
	sourceId string,
	o *IngestEndpointListOptions,
) (*models.ListResponseIngestEndpointOut, error) {
	pathMap := map[string]string{
		"source_id": sourceId,
	}
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
	return internal.ExecuteRequest[any, models.ListResponseIngestEndpointOut](
		ctx,
		ingestEndpoint.client,
		"GET",
		"/ingest/api/v1/source/{source_id}/endpoint",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Create an ingest endpoint.
func (ingestEndpoint *IngestEndpoint) Create(
	ctx context.Context,
	sourceId string,
	ingestEndpointIn models.IngestEndpointIn,
	o *IngestEndpointCreateOptions,
) (*models.IngestEndpointOut, error) {
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.IngestEndpointIn, models.IngestEndpointOut](
		ctx,
		ingestEndpoint.client,
		"POST",
		"/ingest/api/v1/source/{source_id}/endpoint",
		pathMap,
		nil,
		headerMap,
		&ingestEndpointIn,
	)
}

// Get an ingest endpoint.
func (ingestEndpoint *IngestEndpoint) Get(
	ctx context.Context,
	sourceId string,
	endpointId string,
) (*models.IngestEndpointOut, error) {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[any, models.IngestEndpointOut](
		ctx,
		ingestEndpoint.client,
		"GET",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update an ingest endpoint.
func (ingestEndpoint *IngestEndpoint) Update(
	ctx context.Context,
	sourceId string,
	endpointId string,
	ingestEndpointUpdate models.IngestEndpointUpdate,
) (*models.IngestEndpointOut, error) {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[models.IngestEndpointUpdate, models.IngestEndpointOut](
		ctx,
		ingestEndpoint.client,
		"PUT",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		&ingestEndpointUpdate,
	)
}

// Delete an ingest endpoint.
func (ingestEndpoint *IngestEndpoint) Delete(
	ctx context.Context,
	sourceId string,
	endpointId string,
) error {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		ingestEndpoint.client,
		"DELETE",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Get the additional headers to be sent with the ingest.
func (ingestEndpoint *IngestEndpoint) GetHeaders(
	ctx context.Context,
	sourceId string,
	endpointId string,
) (*models.IngestEndpointHeadersOut, error) {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[any, models.IngestEndpointHeadersOut](
		ctx,
		ingestEndpoint.client,
		"GET",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Set the additional headers to be sent to the endpoint.
func (ingestEndpoint *IngestEndpoint) UpdateHeaders(
	ctx context.Context,
	sourceId string,
	endpointId string,
	ingestEndpointHeadersIn models.IngestEndpointHeadersIn,
) error {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	_, err := internal.ExecuteRequest[models.IngestEndpointHeadersIn, any](
		ctx,
		ingestEndpoint.client,
		"PUT",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		&ingestEndpointHeadersIn,
	)
	return err
}

// Get an ingest endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (ingestEndpoint *IngestEndpoint) GetSecret(
	ctx context.Context,
	sourceId string,
	endpointId string,
) (*models.IngestEndpointSecretOut, error) {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[any, models.IngestEndpointSecretOut](
		ctx,
		ingestEndpoint.client,
		"GET",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Rotates an ingest endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (ingestEndpoint *IngestEndpoint) RotateSecret(
	ctx context.Context,
	sourceId string,
	endpointId string,
	ingestEndpointSecretIn models.IngestEndpointSecretIn,
	o *IngestEndpointRotateSecretOptions,
) error {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[models.IngestEndpointSecretIn, any](
		ctx,
		ingestEndpoint.client,
		"POST",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
		pathMap,
		nil,
		headerMap,
		&ingestEndpointSecretIn,
	)
	return err
}
