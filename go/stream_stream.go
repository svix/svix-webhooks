// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type StreamStream struct {
	client *internal.SvixHttpClient
}

func newStreamStream(client *internal.SvixHttpClient) *StreamStream {
	return &StreamStream{
		client: client,
	}
}

type StreamStreamListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type StreamStreamCreateOptions struct {
	IdempotencyKey *string
}

// List of all the organization's streams.
func (streamStream *StreamStream) List(
	ctx context.Context,
	o *StreamStreamListOptions,
) (*models.ListResponseStreamOut, error) {
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
	return internal.ExecuteRequest[any, models.ListResponseStreamOut](
		ctx,
		streamStream.client,
		"GET",
		"/api/v1/stream",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Creates a new stream.
func (streamStream *StreamStream) Create(
	ctx context.Context,
	streamIn models.StreamIn,
	o *StreamStreamCreateOptions,
) (*models.StreamOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.StreamIn, models.StreamOut](
		ctx,
		streamStream.client,
		"POST",
		"/api/v1/stream",
		nil,
		nil,
		headerMap,
		&streamIn,
	)
}

// Get a stream by id or uid.
func (streamStream *StreamStream) Get(
	ctx context.Context,
	streamId string,
) (*models.StreamOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	return internal.ExecuteRequest[any, models.StreamOut](
		ctx,
		streamStream.client,
		"GET",
		"/api/v1/stream/{stream_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update a stream.
func (streamStream *StreamStream) Update(
	ctx context.Context,
	streamId string,
	streamIn models.StreamIn,
) (*models.StreamOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	return internal.ExecuteRequest[models.StreamIn, models.StreamOut](
		ctx,
		streamStream.client,
		"PUT",
		"/api/v1/stream/{stream_id}",
		pathMap,
		nil,
		nil,
		&streamIn,
	)
}

// Delete a stream.
func (streamStream *StreamStream) Delete(
	ctx context.Context,
	streamId string,
) error {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		streamStream.client,
		"DELETE",
		"/api/v1/stream/{stream_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Partially update a stream.
func (streamStream *StreamStream) Patch(
	ctx context.Context,
	streamId string,
	streamPatch models.StreamPatch,
) (*models.StreamOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	return internal.ExecuteRequest[models.StreamPatch, models.StreamOut](
		ctx,
		streamStream.client,
		"PATCH",
		"/api/v1/stream/{stream_id}",
		pathMap,
		nil,
		nil,
		&streamPatch,
	)
}
