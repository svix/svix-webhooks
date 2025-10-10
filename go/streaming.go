// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Streaming struct {
	client    *internal.SvixHttpClient
	EventType *StreamingEventType
	Events    *StreamingEvents
	Sink      *StreamingSink
	Stream    *StreamingStream
}

func newStreaming(client *internal.SvixHttpClient) *Streaming {
	return &Streaming{
		client:    client,
		EventType: newStreamingEventType(client),
		Events:    newStreamingEvents(client),
		Sink:      newStreamingSink(client),
		Stream:    newStreamingStream(client),
	}
}

// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
func (streaming *Streaming) SinkHeadersGet(
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
		streaming.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
func (streaming *Streaming) SinkHeadersPatch(
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
		streaming.client,
		"PATCH",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
		pathMap,
		nil,
		nil,
		&httpSinkHeadersPatchIn,
	)
}

// Get the transformation code associated with this sink.
func (streaming *Streaming) SinkTransformationGet(
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
		streaming.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}
