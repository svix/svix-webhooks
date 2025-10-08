// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamSink struct {
	client *internal.SvixHttpClient
}

func newStreamSink(client *internal.SvixHttpClient) *StreamSink {
	return &StreamSink{
		client: client,
	}
}

type StreamSinkListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type StreamSinkCreateOptions struct {
	IdempotencyKey *string
}

type StreamSinkRotateSecretOptions struct {
	IdempotencyKey *string
}

// List of all the stream's sinks.
func (streamSink *StreamSink) List(
	ctx context.Context,
	streamId string,
	o *StreamSinkListOptions,
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
		streamSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Creates a new sink.
func (streamSink *StreamSink) Create(
	ctx context.Context,
	streamId string,
	streamSinkIn models.StreamSinkIn,
	o *StreamSinkCreateOptions,
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
		streamSink.client,
		"POST",
		"/api/v1/stream/{stream_id}/sink",
		pathMap,
		nil,
		headerMap,
		&streamSinkIn,
	)
}

// Get a sink by id or uid.
func (streamSink *StreamSink) Get(
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
		streamSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update a sink.
func (streamSink *StreamSink) Update(
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
		streamSink.client,
		"PUT",
		"/api/v1/stream/{stream_id}/sink/{sink_id}",
		pathMap,
		nil,
		nil,
		&streamSinkIn,
	)
}

// Delete a sink.
func (streamSink *StreamSink) Delete(
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
		streamSink.client,
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
func (streamSink *StreamSink) Patch(
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
		streamSink.client,
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
func (streamSink *StreamSink) GetSecret(
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
		streamSink.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Rotates the signing secret (only supported for http sinks).
func (streamSink *StreamSink) RotateSecret(
	ctx context.Context,
	streamId string,
	sinkId string,
	endpointSecretRotateIn models.EndpointSecretRotateIn,
	o *StreamSinkRotateSecretOptions,
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
		streamSink.client,
		"POST",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
		pathMap,
		nil,
		headerMap,
		&endpointSecretRotateIn,
	)
}

// Set or unset the transformation code associated with this sink.
func (streamSink *StreamSink) TransformationPartialUpdate(
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
		streamSink.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		&sinkTransformIn,
	)
}
