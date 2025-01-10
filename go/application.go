// this file is @generated
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Application struct {
	api *openapi.APIClient
}

type ApplicationListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

// List of all the organization's applications.
func (application *Application) List(
	ctx context.Context,
	options *ApplicationListOptions,
) (*ListResponseApplicationOut, error) {
	req := application.api.ApplicationAPI.V1ApplicationList(
		ctx,
	)

	if options != nil {
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Create a new application.
func (application *Application) Create(
	ctx context.Context,
	applicationIn *ApplicationIn,
) (*ApplicationOut, error) {
	return application.CreateWithOptions(
		ctx,
		applicationIn,
		nil,
	)
}

// Create a new application.
func (application *Application) CreateWithOptions(
	ctx context.Context,
	applicationIn *ApplicationIn,
	options *PostOptions,
) (*ApplicationOut, error) {
	req := application.api.ApplicationAPI.V1ApplicationCreate(
		ctx,
	).ApplicationIn(*applicationIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Get an application.
func (application *Application) Get(
	ctx context.Context,
	appId string,
) (*ApplicationOut, error) {
	req := application.api.ApplicationAPI.V1ApplicationGet(
		ctx,
		appId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Update an application.
func (application *Application) Update(
	ctx context.Context,
	appId string,
	applicationIn *ApplicationIn,
) (*ApplicationOut, error) {
	req := application.api.ApplicationAPI.V1ApplicationUpdate(
		ctx,
		appId,
	).ApplicationIn(*applicationIn)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Delete an application.
func (application *Application) Delete(
	ctx context.Context,
	appId string,
) error {
	req := application.api.ApplicationAPI.V1ApplicationDelete(
		ctx,
		appId,
	)

	res, err := req.Execute()
	return wrapError(err, res)
}

// Partially update an application.
func (application *Application) Patch(
	ctx context.Context,
	appId string,
	applicationPatch *ApplicationPatch,
) (*ApplicationOut, error) {
	req := application.api.ApplicationAPI.V1ApplicationPatch(
		ctx,
		appId,
	).ApplicationPatch(*applicationPatch)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}
