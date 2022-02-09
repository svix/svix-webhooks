package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseApplicationOut openapi.ListResponseApplicationOut
	ApplicationIn              openapi.ApplicationIn
	ApplicationOut             openapi.ApplicationOut
)

type Application struct {
	api *openapi.APIClient
}

type ApplicationListOptions struct {
	Iterator *string
	Limit    *int32
}

func (a *Application) List(options *ApplicationListOptions) (*ListResponseApplicationOut, error) {
	req := a.api.ApplicationApi.ListApplicationsApiV1AppGet(context.Background())
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Create(applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.CreateApplicationApiV1AppPost(context.Background())
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Get(appId string) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.GetApplicationApiV1AppAppIdGet(context.Background(), appId)
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Update(appId string, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.UpdateApplicationApiV1AppAppIdPut(context.Background(), appId)
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Delete(appId string) error {
	req := a.api.ApplicationApi.DeleteApplicationApiV1AppAppIdDelete(context.Background(), appId)
	res, err := req.Execute()
	return wrapError(err, res)
}
