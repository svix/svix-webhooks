// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamingEventType struct {
	client *internal.SvixHttpClient
}

func newStreamingEventType(client *internal.SvixHttpClient) *StreamingEventType {
	return &StreamingEventType{
		client: client,
	}
}

type StreamingEventTypeListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
	// Include archived (deleted but not expunged) items in the response.
	IncludeArchived *bool
}

type StreamingEventTypeCreateOptions struct {
	IdempotencyKey *string
}

type StreamingEventTypeDeleteOptions struct {
	// By default, event types are archived when "deleted". With this flag, they are deleted entirely.
	Expunge *bool
}

// List of all the organization's event types for streaming.
func (streamingEventType *StreamingEventType) List(
	ctx context.Context,
	o *StreamingEventTypeListOptions,
) (*models.ListResponseStreamEventTypeOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("order", o.Order, queryMap, &err)
		internal.SerializeParamToMap("include_archived", o.IncludeArchived, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseStreamEventTypeOut](
		ctx,
		streamingEventType.client,
		"GET",
		"/api/v1/stream/event-type",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create an event type for Streams.
func (streamingEventType *StreamingEventType) Create(
	ctx context.Context,
	streamEventTypeIn models.StreamEventTypeIn,
	o *StreamingEventTypeCreateOptions,
) (*models.StreamEventTypeOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.StreamEventTypeIn, models.StreamEventTypeOut](
		ctx,
		streamingEventType.client,
		"POST",
		"/api/v1/stream/event-type",
		nil,
		nil,
		headerMap,
		&streamEventTypeIn,
	)
}

// Get an event type.
func (streamingEventType *StreamingEventType) Get(
	ctx context.Context,
	name string,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[any, models.StreamEventTypeOut](
		ctx,
		streamingEventType.client,
		"GET",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update or create a event type for Streams.
func (streamingEventType *StreamingEventType) Update(
	ctx context.Context,
	name string,
	streamEventTypeIn models.StreamEventTypeIn,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[models.StreamEventTypeIn, models.StreamEventTypeOut](
		ctx,
		streamingEventType.client,
		"PUT",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		&streamEventTypeIn,
	)
}

// Delete an event type.
func (streamingEventType *StreamingEventType) Delete(
	ctx context.Context,
	name string,
	o *StreamingEventTypeDeleteOptions,
) error {
	pathMap := map[string]string{
		"name": name,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("expunge", o.Expunge, queryMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		streamingEventType.client,
		"DELETE",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
	return err
}

// Patch an event type for Streams.
func (streamingEventType *StreamingEventType) Patch(
	ctx context.Context,
	name string,
	streamEventTypePatch models.StreamEventTypePatch,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[models.StreamEventTypePatch, models.StreamEventTypeOut](
		ctx,
		streamingEventType.client,
		"PATCH",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		&streamEventTypePatch,
	)
}
