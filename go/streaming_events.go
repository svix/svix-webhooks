// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamingEvents struct {
	client *internal.SvixHttpClient
}

func newStreamingEvents(client *internal.SvixHttpClient) *StreamingEvents {
	return &StreamingEvents{
		client: client,
	}
}

type StreamingEventsCreateOptions struct {
	IdempotencyKey *string
}

type StreamingEventsGetOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	After    *time.Time
}

// Creates events on the Stream.
func (streamingEvents *StreamingEvents) Create(
	ctx context.Context,
	streamId string,
	createStreamEventsIn models.CreateStreamEventsIn,
	o *StreamingEventsCreateOptions,
) (*models.CreateStreamEventsOut, error) {
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
	return internal.ExecuteRequest[models.CreateStreamEventsIn, models.CreateStreamEventsOut](
		ctx,
		streamingEvents.client,
		"POST",
		"/api/v1/stream/{stream_id}/events",
		pathMap,
		nil,
		headerMap,
		&createStreamEventsIn,
	)
}

// Iterate over a stream of events.
//
// The sink must be of type `poller` to use the poller endpoint.
func (streamingEvents *StreamingEvents) Get(
	ctx context.Context,
	streamId string,
	sinkId string,
	o *StreamingEventsGetOptions,
) (*models.EventStreamOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("after", o.After, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.EventStreamOut](
		ctx,
		streamingEvents.client,
		"GET",
		"/api/v1/stream/{stream_id}/sink/{sink_id}/events",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}
