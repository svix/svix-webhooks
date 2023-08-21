package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseIntegrationOut = openapi.ListResponseIntegrationOut
	IntegrationIn              = openapi.IntegrationIn
	IntegrationUpdate          = openapi.IntegrationUpdate
	IntegrationOut             = openapi.IntegrationOut
	IntegrationKeyOut          = openapi.IntegrationKeyOut
)

type Integration struct {
	api *openapi.APIClient
}

type IntegrationListOptions struct {
	Iterator *string
	Limit    *int32
}

func (e *Integration) List(ctx context.Context, appId string, options *IntegrationListOptions) (*ListResponseIntegrationOut, error) {
	req := e.api.IntegrationApi.V1IntegrationList(ctx, appId)
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
	ret := ListResponseIntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Create(ctx context.Context, appId string, endpointIn *IntegrationIn) (*IntegrationOut, error) {
	return e.CreateWithOptions(ctx, appId, endpointIn, nil)
}

func (e *Integration) CreateWithOptions(ctx context.Context, appId string, endpointIn *IntegrationIn, options *PostOptions) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.V1IntegrationCreate(ctx, appId)
	req = req.IntegrationIn(openapi.IntegrationIn(*endpointIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Get(ctx context.Context, appId string, integId string) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.V1IntegrationGet(ctx, appId, integId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Update(ctx context.Context, appId string, integId string, endpointUpdate *IntegrationUpdate) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.V1IntegrationUpdate(ctx, appId, integId)
	req = req.IntegrationUpdate(openapi.IntegrationUpdate(*endpointUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Delete(ctx context.Context, appId string, integId string) error {
	req := e.api.IntegrationApi.V1IntegrationDelete(ctx, appId, integId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Integration) GetKey(ctx context.Context, appId string, integId string) (*IntegrationKeyOut, error) {
	req := e.api.IntegrationApi.V1IntegrationGetKey(ctx, appId, integId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationKeyOut(out)
	return &ret, nil
}

func (e *Integration) RotateKey(ctx context.Context, appId string, integId string) (*IntegrationKeyOut, error) {
	return e.RotateKeyWithOptions(ctx, appId, integId, nil)
}

func (e *Integration) RotateKeyWithOptions(ctx context.Context, appId string, integId string, options *PostOptions) (*IntegrationKeyOut, error) {
	req := e.api.IntegrationApi.V1IntegrationRotateKey(ctx, appId, integId)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationKeyOut(out)
	return &ret, nil
}
