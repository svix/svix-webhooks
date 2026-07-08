// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type EndpointTransformation struct {
	client *internal.SvixHttpClient
}

func newEndpointTransformation(client *internal.SvixHttpClient) EndpointTransformation {
	return EndpointTransformation{client}
}

// Get the transformation code associated with this endpoint.
func (endpointTransformation EndpointTransformation) Get(
	ctx context.Context,
	appId string,
	endpointId string,
) (*models.EndpointTransformationOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	return internal.ExecuteRequest[any, models.EndpointTransformationOut](
		ctx,
		endpointTransformation.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Set or unset the transformation code associated with this endpoint.
func (endpointTransformation EndpointTransformation) Patch(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointTransformationPatch models.EndpointTransformationPatch,
) error {
	var err error
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	_, err = internal.ExecuteRequest[models.EndpointTransformationPatch, any](
		ctx,
		endpointTransformation.client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		&endpointTransformationPatch,
	)
	return err
}
