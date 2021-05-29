package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type (
	ListResponseEndpointOut openapi.ListResponseEndpointOut
	EndpointIn              openapi.EndpointIn
	EndpointOut             openapi.EndpointOut
	EndpointSecret          openapi.EndpointSecret
)

type Endpoint struct {
	api *openapi.APIClient
}

func (e *Endpoint) List(appId string, options *FetchOptions) (*ListResponseEndpointOut, error) {
	req := e.api.EndpointApi.ListEndpointsApiV1AppAppIdEndpointGet(context.Background(), appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseEndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Create(appId string, endpointIn *EndpointIn) (*EndpointOut, error) {
	req := e.api.EndpointApi.CreateEndpointApiV1AppAppIdEndpointPost(context.Background(), appId)
	req = req.EndpointIn(openapi.EndpointIn(*endpointIn))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Get(appId string, endpointId string) (*EndpointOut, error) {
	req := e.api.EndpointApi.GetEndpointApiV1AppAppIdEndpointEndpointIdGet(context.Background(), endpointId, appId)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Update(appId string, endpointId string, endpointIn *EndpointIn) (*EndpointOut, error) {
	req := e.api.EndpointApi.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPut(context.Background(), endpointId, appId)
	req = req.EndpointIn(openapi.EndpointIn(*endpointIn))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Delete(appId string, endpointId string) error {
	req := e.api.EndpointApi.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(context.Background(), endpointId, appId)
	_, err := req.Execute()
	return err
}

func (e *Endpoint) GetSecret(appId string, endpointId string) (*EndpointSecret, error) {
	req := e.api.EndpointApi.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(context.Background(), endpointId, appId)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EndpointSecret(out)
	return &ret, nil
}
