// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamingSink struct {
	client *internal.SvixHttpClient
}

func newStreamingSink(client *internal.SvixHttpClient) *StreamingSink {
	return &StreamingSink{
		client: client,
	}
}

type StreamingSinkListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type StreamingSinkCreateOptions struct {
	IdempotencyKey *string
}

type StreamingSinkRotateSecretOptions struct {
	IdempotencyKey *string
}

// List of all the stream's sinks.
func (streamingSink *StreamingSink) List(
	ctx context.Context,
	streamId string,
	o *StreamingSinkListOptions,
) (*models.ListResponseStreamSinkOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
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
	return internal.ExecuteRequest[any, models.ListResponseStreamSinkOut](
		ctx,
		streamingSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Creates a new sink.
func (streamingSink *StreamingSink) Create(
	ctx context.Context,
	streamId string,
	streamSinkIn models.StreamSinkIn,
	o *StreamingSinkCreateOptions,
) (*models.StreamSinkOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.StreamSinkIn, models.StreamSinkOut](
		ctx,
		streamingSink.client,
		"POST",
		"/api/v1/stream/{stream_id}/sink",
		pathMap,
		nil,
		headerMap,
		&streamSinkIn,
	)
}

// Get a sink by id or uid.
func (streamingSink *StreamingSink) Get(
	ctx context.Context,
	streamId string,
	sinkId string,
) (*models.StreamSinkOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[any, models.StreamSinkOut](
		ctx,
		streamingSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update a sink.
func (streamingSink *StreamingSink) Update(
	ctx context.Context,
	streamId string,
	sinkId string,
	streamSinkIn models.StreamSinkIn,
) (*models.StreamSinkOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[models.StreamSinkIn, models.StreamSinkOut](
		ctx,
		streamingSink.client,
		"PUT",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		&streamSinkIn,
	)
}

// Delete a sink.
func (streamingSink *StreamingSink) Delete(
	ctx context.Context,
	streamId string,
	sinkId string,
) error {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		streamingSink.client,
		"DELETE",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Partially update a sink.
func (streamingSink *StreamingSink) Patch(
	ctx context.Context,
	streamId string,
	sinkId string,
	streamSinkPatch models.StreamSinkPatch,
) (*models.StreamSinkOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[models.StreamSinkPatch, models.StreamSinkOut](
		ctx,
		streamingSink.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		&streamSinkPatch,
	)
}

// Get the sink's signing secret (only supported for http sinks)
//
// This is used to verify the authenticity of the delivery.
//
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (streamingSink *StreamingSink) GetSecret(
	ctx context.Context,
	streamId string,
	sinkId string,
) (*models.SinkSecretOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[any, models.SinkSecretOut](
		ctx,
		streamingSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Rotates the signing secret (only supported for http sinks).
func (streamingSink *StreamingSink) RotateSecret(
	ctx context.Context,
	streamId string,
	sinkId string,
	endpointSecretRotateIn models.EndpointSecretRotateIn,
	o *StreamingSinkRotateSecretOptions,
) (*models.EmptyResponse, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.EndpointSecretRotateIn, models.EmptyResponse](
		ctx,
		streamingSink.client,
		"POST",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
		pathMap,
		nil,
		headerMap,
		&endpointSecretRotateIn,
	)
}

// Set or unset the transformation code associated with this sink.
func (streamingSink *StreamingSink) TransformationPartialUpdate(
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
		streamingSink.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		&sinkTransformIn,
	)
}
