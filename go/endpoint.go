package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseEndpointOut openapi.ListResponseEndpointOut
	EndpointIn              openapi.EndpointIn
	EndpointUpdate          openapi.EndpointUpdate
	EndpointOut             openapi.EndpointOut
	EndpointSecretOut       openapi.EndpointSecretOut
	EndpointSecretRotateIn  openapi.EndpointSecretRotateIn
	RecoverIn               openapi.RecoverIn
	EndpointHeadersIn       openapi.EndpointHeadersIn
	EndpointHeadersPatchIn  openapi.EndpointHeadersPatchIn
	EndpointHeadersOut      openapi.EndpointHeadersOut
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
	return e.CreateWithOptions(appId, endpointIn, nil)
}

func (e *Endpoint) CreateWithOptions(appId string, endpointIn *EndpointIn, options *PostOptions) (*EndpointOut, error) {
	req := e.api.EndpointApi.CreateEndpointApiV1AppAppIdEndpointPost(context.Background(), appId)
	req = req.EndpointIn(openapi.EndpointIn(*endpointIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
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

func (e *Endpoint) RotateSecret(appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn) error {
	return e.RotateSecretWithOptions(appId, endpointId, endpointSecretRotateIn, nil)
}

func (e *Endpoint) RotateSecretWithOptions(appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn, options *PostOptions) error {
	req := e.api.EndpointApi.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePost(context.Background(), endpointId, appId)
	req = req.EndpointSecretRotateIn(openapi.EndpointSecretRotateIn(*endpointSecretRotateIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) Recover(appId string, endpointId string, recoverIn *RecoverIn) error {
	return e.RecoverWithOptions(appId, endpointId, recoverIn, nil)
}

func (e *Endpoint) RecoverWithOptions(appId string, endpointId string, recoverIn *RecoverIn, options *PostOptions) error {
	req := e.api.EndpointApi.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPost(context.Background(), appId, endpointId)
	req = req.RecoverIn(openapi.RecoverIn(*recoverIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	_, res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) GetHeaders(appId string, endpointId string) (*EndpointHeadersOut, error) {
	req := e.api.EndpointApi.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(context.Background(), endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointHeadersOut(out)
	return &ret, nil
}

func (e *Endpoint) UpdateHeaders(appId string, endpointId string, endpointHeadersIn *EndpointHeadersIn) error {
	req := e.api.EndpointApi.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPut(context.Background(), appId, endpointId)
	req = req.EndpointHeadersIn(openapi.EndpointHeadersIn(*endpointHeadersIn))
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) PatchHeaders(appId string, endpointId string, endpointHeadersIn *EndpointHeadersPatchIn) error {
	req := e.api.EndpointApi.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatch(context.Background(), appId, endpointId)
	req = req.EndpointHeadersPatchIn(openapi.EndpointHeadersPatchIn(*endpointHeadersIn))
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}
