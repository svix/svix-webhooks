// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamingSinkTransformation struct {
	client *internal.SvixHttpClient
}

func newStreamingSinkTransformation(client *internal.SvixHttpClient) StreamingSinkTransformation {
	return StreamingSinkTransformation{client}
}

// Get the transformation code associated with this sink.
func (streamingSinkTransformation StreamingSinkTransformation) Get(
	ctx context.Context,
	streamId string,
	sinkId string,
) (*models.SinkTransformationOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[any, models.SinkTransformationOut](
		ctx,
		streamingSinkTransformation.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Set or unset the transformation code associated with this sink.
func (streamingSinkTransformation StreamingSinkTransformation) Patch(
	ctx context.Context,
	streamId string,
	sinkId string,
	sinkTransformIn models.SinkTransformIn,
) (*models.EmptyResponse, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[models.SinkTransformIn, models.EmptyResponse](
		ctx,
		streamingSinkTransformation.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		&sinkTransformIn,
	)
}
