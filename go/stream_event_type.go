// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamEventType struct {
	client *internal.SvixHttpClient
}

func newStreamEventType(client *internal.SvixHttpClient) *StreamEventType {
	return &StreamEventType{
		client: client,
	}
}

type StreamEventTypeListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
	// Include archived (deleted but not expunged) items in the response.
	IncludeArchived *bool
}

type StreamEventTypeCreateOptions struct {
	IdempotencyKey *string
}

type StreamEventTypeDeleteOptions struct {
	// By default, event types are archived when "deleted". With this flag, they are deleted entirely.
	Expunge *bool
}

// List of all the organization's event types for streaming.
func (streamEventType *StreamEventType) List(
	ctx context.Context,
	o *StreamEventTypeListOptions,
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
		streamEventType.client,
		"GET",
		"/api/v1/stream/event-type",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create an event type for Streams.
func (streamEventType *StreamEventType) Create(
	ctx context.Context,
	streamEventTypeIn models.StreamEventTypeIn,
	o *StreamEventTypeCreateOptions,
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
		streamEventType.client,
		"POST",
		"/api/v1/stream/event-type",
		nil,
		nil,
		headerMap,
		&streamEventTypeIn,
	)
}

// Get an event type.
func (streamEventType *StreamEventType) Get(
	ctx context.Context,
	name string,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[any, models.StreamEventTypeOut](
		ctx,
		streamEventType.client,
		"GET",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update or create a event type for Streams.
func (streamEventType *StreamEventType) Update(
	ctx context.Context,
	name string,
	streamEventTypeIn models.StreamEventTypeIn,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[models.StreamEventTypeIn, models.StreamEventTypeOut](
		ctx,
		streamEventType.client,
		"PUT",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		&streamEventTypeIn,
	)
}

// Delete an event type.
func (streamEventType *StreamEventType) Delete(
	ctx context.Context,
	name string,
	o *StreamEventTypeDeleteOptions,
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
		streamEventType.client,
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
func (streamEventType *StreamEventType) Patch(
	ctx context.Context,
	name string,
	streamEventTypePatch models.StreamEventTypePatch,
) (*models.StreamEventTypeOut, error) {
	pathMap := map[string]string{
		"name": name,
	}
	return internal.ExecuteRequest[models.StreamEventTypePatch, models.StreamEventTypeOut](
		ctx,
		streamEventType.client,
		"PATCH",
		"/api/v1/stream/event-type/{name}",
		pathMap,
		nil,
		nil,
		&streamEventTypePatch,
	)
}
