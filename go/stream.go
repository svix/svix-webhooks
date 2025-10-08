// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Stream struct {
	client    *internal.SvixHttpClient
	EventType *StreamEventType
	Events    *StreamEvents
	Sink      *StreamSink
	Stream    *StreamStream
}

func newStream(client *internal.SvixHttpClient) *Stream {
	return &Stream{
		client:    client,
		EventType: newStreamEventType(client),
		Events:    newStreamEvents(client),
		Sink:      newStreamSink(client),
		Stream:    newStreamStream(client),
	}
}

// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
func (stream *Stream) SinkHeadersGet(
	ctx context.Context,
	streamId string,
	sinkId string,
) (*models.EndpointHeadersOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[any, models.EndpointHeadersOut](
		ctx,
		stream.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
func (stream *Stream) SinkHeadersPatch(
	ctx context.Context,
	streamId string,
	sinkId string,
	httpSinkHeadersPatchIn models.HttpSinkHeadersPatchIn,
) (*models.EndpointHeadersOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[models.HttpSinkHeadersPatchIn, models.EndpointHeadersOut](
		ctx,
		stream.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
		pathMap,
		nil,
		nil,
		&httpSinkHeadersPatchIn,
	)
}

// Get the transformation code associated with this sink.
func (stream *Stream) SinkTransformationGet(
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
		stream.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}
