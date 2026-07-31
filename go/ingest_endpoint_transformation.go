// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type IngestEndpointTransformation struct {
	client *internal.SvixHttpClient
}

func newIngestEndpointTransformation(client *internal.SvixHttpClient) IngestEndpointTransformation {
	return IngestEndpointTransformation{client}
}

// Get the transformation code associated with this ingest endpoint.
func (ingestEndpointTransformation IngestEndpointTransformation) Transformation(
	ctx context.Context,
	sourceId string,
	endpointId string,
) (*models.IngestEndpointTransformationOut, error) {
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[any, models.IngestEndpointTransformationOut](
		ctx,
		ingestEndpointTransformation.client,
		"GET",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Set or unset the transformation code associated with this ingest endpoint.
func (ingestEndpointTransformation IngestEndpointTransformation) Patch(
	ctx context.Context,
	sourceId string,
	endpointId string,
	ingestEndpointTransformationPatch models.IngestEndpointTransformationPatch,
) error {
	var err error
	pathMap := map[string]string{
		"source_id":   sourceId,
		"endpoint_id": endpointId,
	}
	_, err = internal.ExecuteRequest[models.IngestEndpointTransformationPatch, any](
		ctx,
		ingestEndpointTransformation.client,
		"PATCH",
		"/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		&ingestEndpointTransformationPatch,
	)
	return err
}
