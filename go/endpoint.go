package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type (
	ListResponseEndpointOut openapi.ListResponseEndpointOut
	EndpointIn              openapi.EndpointIn
	EndpointUpdate          openapi.EndpointUpdate
	EndpointOut             openapi.EndpointOut
	EndpointSecretOut       openapi.EndpointSecretOut
)

type Endpoint struct {
	api *openapi.APIClient
}

type EndpointListOptions struct {
	Iterator *string
	Limit    *int32
}

func (e *Endpoint) List(appId string, options *EndpointListOptions) (*ListResponseEndpointOut, error) {
	req := e.api.EndpointApi.ListEndpointsApiV1AppAppIdEndpointGet(context.Background(), appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseEndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Create(appId string, endpointIn *EndpointIn) (*EndpointOut, error) {
	req := e.api.EndpointApi.CreateEndpointApiV1AppAppIdEndpointPost(context.Background(), appId)
	req = req.EndpointIn(openapi.EndpointIn(*endpointIn))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Get(appId string, endpointId string) (*EndpointOut, error) {
	req := e.api.EndpointApi.GetEndpointApiV1AppAppIdEndpointEndpointIdGet(context.Background(), endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Update(appId string, endpointId string, endpointUpdate *EndpointUpdate) (*EndpointOut, error) {
	req := e.api.EndpointApi.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPut(context.Background(), endpointId, appId)
	req = req.EndpointUpdate(openapi.EndpointUpdate(*endpointUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Delete(appId string, endpointId string) error {
	req := e.api.EndpointApi.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(context.Background(), endpointId, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Endpoint) GetSecret(appId string, endpointId string) (*EndpointSecretOut, error) {
	req := e.api.EndpointApi.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(context.Background(), endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointSecretOut(out)
	return &ret, nil
}
