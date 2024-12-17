package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseApplicationOut = openapi.ListResponseApplicationOut
	ApplicationIn              = openapi.ApplicationIn
	ApplicationOut             = openapi.ApplicationOut
	ApplicationPatch           = openapi.ApplicationPatch
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

func (a *Application) List(ctx context.Context, options *ApplicationListOptions) (*ListResponseApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationList(ctx)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
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

func (a *Application) Create(ctx context.Context, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	return a.CreateWithOptions(ctx, applicationIn, nil)
}

func (a *Application) CreateWithOptions(ctx context.Context, applicationIn *ApplicationIn, options *PostOptions) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationCreate(ctx)
	req = req.ApplicationIn(*applicationIn)
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

func (a *Application) GetOrCreate(ctx context.Context, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	return a.GetOrCreateWithOptions(ctx, applicationIn, nil)
}

func (a *Application) GetOrCreateWithOptions(ctx context.Context, applicationIn *ApplicationIn, options *PostOptions) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationCreate(ctx)
	req = req.ApplicationIn(*applicationIn)
	req = req.GetIfExists(true)
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

func (a *Application) Get(ctx context.Context, appId string) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationGet(ctx, appId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (a *Application) Update(ctx context.Context, appId string, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationUpdate(ctx, appId)
	req = req.ApplicationIn(*applicationIn)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (a *Application) Patch(ctx context.Context, appId string, applicationPatch *ApplicationPatch) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationPatch(ctx, appId)
	req = req.ApplicationPatch(*applicationPatch)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (a *Application) Delete(ctx context.Context, appId string) error {
	req := a.api.ApplicationAPI.V1ApplicationDelete(ctx, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}
