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
	Iterator *string
	Limit    *int32
	Order    *Ordering
}

func (a *Application) List(ctx context.Context, options *ApplicationListOptions) (*ListResponseApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationList(ctx)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Order != nil {
			req = req.Order(openapi.Ordering(*options.Order))
		}
	}
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Create(ctx context.Context, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	return a.CreateWithOptions(ctx, applicationIn, nil)
}

func (a *Application) CreateWithOptions(ctx context.Context, applicationIn *ApplicationIn, options *PostOptions) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationCreate(ctx)
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) GetOrCreate(ctx context.Context, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	return a.GetOrCreateWithOptions(ctx, applicationIn, nil)
}

func (a *Application) GetOrCreateWithOptions(ctx context.Context, applicationIn *ApplicationIn, options *PostOptions) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationCreate(ctx)
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	req = req.GetIfExists(true)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Get(ctx context.Context, appId string) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationGet(ctx, appId)
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Update(ctx context.Context, appId string, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationUpdate(ctx, appId)
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Patch(ctx context.Context, appId string, applicationPatch *ApplicationPatch) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.V1ApplicationPatch(ctx, appId)
	req = req.ApplicationPatch(openapi.ApplicationPatch(*applicationPatch))
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Delete(ctx context.Context, appId string) error {
	req := a.api.ApplicationApi.V1ApplicationDelete(ctx, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}
