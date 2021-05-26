package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type (
	FetchOptions               struct{}
	ListResponseApplicationOut openapi.ListResponseApplicationOut
	ApplicationIn              openapi.ApplicationIn
	ApplicationOut             openapi.ApplicationOut
)

type Application struct {
	api *openapi.APIClient
}

// func (a *Application) List(options FetchOptions) (*ListResponseApplicationOut, error) {
func (a *Application) List() (*ListResponseApplicationOut, error) {
	req := a.api.ApplicationApi.ListApplicationsApiV1AppGet(context.TODO())
	resp, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Create(applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.CreateApplicationApiV1AppPost(context.TODO())
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	resp, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Get(appID string) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.GetApplicationApiV1AppAppIdGet(context.TODO(), appID)
	resp, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Update(appID string, applicationIn *ApplicationIn) (*ApplicationOut, error) {
	req := a.api.ApplicationApi.UpdateApplicationApiV1AppAppIdPut(context.TODO(), appID)
	req = req.ApplicationIn(openapi.ApplicationIn(*applicationIn))
	resp, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ApplicationOut(resp)
	return &ret, nil
}

func (a *Application) Delete(appID string) error {
	req := a.api.ApplicationApi.DeleteApplicationApiV1AppAppIdDelete(context.TODO(), appID)
	_, err := req.Execute()
	return err
}
