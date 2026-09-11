// Package svix this file is @generated DO NOT EDIT
package api

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Streaming struct {
	client *internal.SvixHttpClient
}

func NewStreaming(client *internal.SvixHttpClient) Streaming {
	return Streaming{client}
}

func (streaming Streaming) EventType() StreamingEventType {
	return NewStreamingEventType(streaming.client)
}
func (streaming Streaming) Events() StreamingEvents {
	return NewStreamingEvents(streaming.client)
}
func (streaming Streaming) Sink() StreamingSink {
	return NewStreamingSink(streaming.client)
}
func (streaming Streaming) Stream() StreamingStream {
	return NewStreamingStream(streaming.client)
}

// Get the HTTP sink headers.
//
// Only valid for `http` or `otelTracing` sinks.
func (streaming Streaming) SinkHeadersGet(
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

// Updates the Sink's headers.
//
// Only valid for `http` or `otelTracing` sinks.
func (streaming Streaming) SinkHeadersPatch(
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
