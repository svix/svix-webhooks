// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type DestinationTransformation struct {
	client *internal.SvixHttpClient
}

func newDestinationTransformation(client *internal.SvixHttpClient) DestinationTransformation {
	return DestinationTransformation{client}
}

// Get the transformation code associated with this destination.
func (destinationTransformation DestinationTransformation) Get(
	ctx context.Context,
	appId string,
	destinationId string,
) (*models.DestinationTransformationOut, error) {
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	return internal.ExecuteRequest[any, models.DestinationTransformationOut](
		ctx,
		destinationTransformation.client,
		"GET",
		"/api/v1/app/{app_id}/destination/{destination_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Set or unset the transformation code associated with this destination.
func (destinationTransformation DestinationTransformation) Patch(
	ctx context.Context,
	appId string,
	destinationId string,
	destinationTransformIn models.DestinationTransformIn,
) (*models.EmptyResponse, error) {
	pathMap := map[string]string{
		"app_id":         appId,
		"destination_id": destinationId,
	}
	return internal.ExecuteRequest[models.DestinationTransformIn, models.EmptyResponse](
		ctx,
		destinationTransformation.client,
		"PATCH",
		"/api/v1/app/{app_id}/destination/{destination_id}/transformation",
		pathMap,
		nil,
		nil,
		&destinationTransformIn,
	)
}
