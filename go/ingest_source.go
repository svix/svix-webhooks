// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type IngestSource struct {
	client *internal.SvixHttpClient
}

func newIngestSource(client *internal.SvixHttpClient) IngestSource {
	return IngestSource{client}
}

type IngestSourceListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type IngestSourceCreateOptions struct {
	IdempotencyKey *string
}

type IngestSourceRotateTokenOptions struct {
	IdempotencyKey *string
}

// List of all the organization's Ingest Sources.
func (ingestSource IngestSource) List(
	ctx context.Context,
	o *IngestSourceListOptions,
) (*models.ListResponseIngestSourceOut, error) {
	var err error
	queryMap := map[string]string{}
	if o == nil {
		opts := IngestSourceListOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
	internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
	internal.SerializeParamToMap("order", o.Order, queryMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[any, models.ListResponseIngestSourceOut](
		ctx,
		ingestSource.client,
		"GET",
		"/ingest/api/v1/source",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create Ingest Source.
func (ingestSource IngestSource) Create(
	ctx context.Context,
	ingestSourceIn models.IngestSourceIn,
	o *IngestSourceCreateOptions,
) (*models.IngestSourceOut, error) {
	var err error
	headerMap := map[string]string{}
	if o == nil {
		opts := IngestSourceCreateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.IngestSourceIn, models.IngestSourceOut](
		ctx,
		ingestSource.client,
		"POST",
		"/ingest/api/v1/source",
		nil,
		nil,
		headerMap,
		&ingestSourceIn,
	)
}

// Get an Ingest Source by id or uid.
func (ingestSource IngestSource) Get(
	ctx context.Context,
	sourceId string,
) (*models.IngestSourceOut, error) {
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	return internal.ExecuteRequest[any, models.IngestSourceOut](
		ctx,
		ingestSource.client,
		"GET",
		"/ingest/api/v1/source/{source_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Create or update an Ingest Source.
func (ingestSource IngestSource) Upsert(
	ctx context.Context,
	sourceId string,
	ingestSourceIn models.IngestSourceIn,
) (*models.IngestSourceOut, error) {
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	return internal.ExecuteRequest[models.IngestSourceIn, models.IngestSourceOut](
		ctx,
		ingestSource.client,
		"PUT",
		"/ingest/api/v1/source/{source_id}",
		pathMap,
		nil,
		nil,
		&ingestSourceIn,
	)
}

// Delete an Ingest Source.
func (ingestSource IngestSource) Delete(
	ctx context.Context,
	sourceId string,
) error {
	var err error
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		ingestSource.client,
		"DELETE",
		"/ingest/api/v1/source/{source_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Rotate the Ingest Source's Url Token.
//
// This will rotate the ingest source's token, which is used to
// construct the unique `ingestUrl` for the source. Previous tokens
// will remain valid for 48 hours after rotation. The token can be
// rotated a maximum of three times within the 48-hour period.
func (ingestSource IngestSource) RotateToken(
	ctx context.Context,
	sourceId string,
	o *IngestSourceRotateTokenOptions,
) (*models.RotateTokenOut, error) {
	var err error
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := IngestSourceRotateTokenOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[any, models.RotateTokenOut](
		ctx,
		ingestSource.client,
		"POST",
		"/ingest/api/v1/source/{source_id}/token/rotate",
		pathMap,
		nil,
		headerMap,
		nil,
	)
}
