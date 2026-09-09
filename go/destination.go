// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Destination struct {
	client *internal.SvixHttpClient
}

func newDestination(client *internal.SvixHttpClient) Destination {
	return Destination{client}
}

func (destination Destination) Transformation() DestinationTransformation {
	return newDestinationTransformation(destination.client)
}

type DestinationListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type DestinationCreateOptions struct {
	IdempotencyKey *string
}

// List of all the application's destinations.
func (destination Destination) List(
	ctx context.Context,
	appId string,
	o *DestinationListOptions,
) (*models.ListResponseDestinationOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	if o == nil {
		opts := DestinationListOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
	internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
	internal.SerializeParamToMap("order", o.Order, queryMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[any, models.ListResponseDestinationOut](
		ctx,
		destination.client,
		"GET",
		"/api/v1/app/{app_id}/destination",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Creates a new destination.
func (destination Destination) Create(
	ctx context.Context,
	appId string,
	destinationIn models.DestinationIn,
	o *DestinationCreateOptions,
) (*models.DestinationOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := DestinationCreateOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.DestinationIn, models.DestinationOut](
		ctx,
		destination.client,
		"POST",
		"/api/v1/app/{app_id}/destination",
		pathMap,
		nil,
		headerMap,
		&destinationIn,
	)
}

// Get a destination by id or uid.
func (destination Destination) Get(
	ctx context.Context,
	appId string,
	destinationId string,
) (*models.DestinationOut, error) {
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	return internal.ExecuteRequest[any, models.DestinationOut](
		ctx,
		destination.client,
		"GET",
		"/api/v1/app/{app_id}/destination/{destination_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Create or update a destination.
func (destination Destination) Upsert(
	ctx context.Context,
	appId string,
	destinationId string,
	destinationIn models.DestinationIn,
) (*models.DestinationOut, error) {
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	return internal.ExecuteRequest[models.DestinationIn, models.DestinationOut](
		ctx,
		destination.client,
		"PUT",
		"/api/v1/app/{app_id}/destination/{destination_id}",
		pathMap,
		nil,
		nil,
		&destinationIn,
	)
}

// Delete a destination.
func (destination Destination) Delete(
	ctx context.Context,
	appId string,
	destinationId string,
) error {
	var err error
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		destination.client,
		"DELETE",
		"/api/v1/app/{app_id}/destination/{destination_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Partially update a destination.
func (destination Destination) Patch(
	ctx context.Context,
	appId string,
	destinationId string,
	destinationPatch models.DestinationPatch,
) (*models.DestinationOut, error) {
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	return internal.ExecuteRequest[models.DestinationPatch, models.DestinationOut](
		ctx,
		destination.client,
		"PATCH",
		"/api/v1/app/{app_id}/destination/{destination_id}",
		pathMap,
		nil,
		nil,
		&destinationPatch,
	)
}
