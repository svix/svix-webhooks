package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseOperationalWebhookEndpointOut = openapi.ListResponseOperationalWebhookEndpointOut
	OperationalWebhookEndpointIn              = openapi.OperationalWebhookEndpointIn
	OperationalWebhookEndpointUpdate          = openapi.OperationalWebhookEndpointUpdate
	OperationalWebhookEndpointOut             = openapi.OperationalWebhookEndpointOut
	OperationalWebhookEndpointSecretOut       = openapi.OperationalWebhookEndpointSecretOut
	OperationalWebhookEndpointSecretIn        = openapi.OperationalWebhookEndpointSecretIn
)

type OperationalWebhookEndpoint struct {
	api *openapi.APIClient
}

type OperationalWebhookEndpointListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

func (e *OperationalWebhookEndpoint) List(ctx context.Context, options *OperationalWebhookEndpointListOptions) (*ListResponseOperationalWebhookEndpointOut, error) {
	req := e.api.WebhookEndpointAPI.ListOperationalWebhookEndpoints(ctx)
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

func (e *OperationalWebhookEndpoint) Create(ctx context.Context, endpointIn *OperationalWebhookEndpointIn) (*OperationalWebhookEndpointOut, error) {
	return e.CreateWithOptions(ctx, endpointIn, nil)
}

func (e *OperationalWebhookEndpoint) CreateWithOptions(ctx context.Context, endpointIn *OperationalWebhookEndpointIn, options *PostOptions) (*OperationalWebhookEndpointOut, error) {
	req := e.api.WebhookEndpointAPI.CreateOperationalWebhookEndpoint(ctx)
	req = req.OperationalWebhookEndpointIn(*endpointIn)
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

func (e *OperationalWebhookEndpoint) Get(ctx context.Context, endpointId string) (*OperationalWebhookEndpointOut, error) {
	req := e.api.WebhookEndpointAPI.GetOperationalWebhookEndpoint(ctx, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *OperationalWebhookEndpoint) Update(ctx context.Context, endpointId string, endpointUpdate *OperationalWebhookEndpointUpdate) (*OperationalWebhookEndpointOut, error) {
	req := e.api.WebhookEndpointAPI.UpdateOperationalWebhookEndpoint(ctx, endpointId)
	req = req.OperationalWebhookEndpointUpdate(*endpointUpdate)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *OperationalWebhookEndpoint) Delete(ctx context.Context, endpointId string) error {
	req := e.api.WebhookEndpointAPI.DeleteOperationalWebhookEndpoint(ctx, endpointId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *OperationalWebhookEndpoint) GetSecret(ctx context.Context, endpointId string) (*OperationalWebhookEndpointSecretOut, error) {
	req := e.api.WebhookEndpointAPI.GetOperationalWebhookEndpointSecret(ctx, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *OperationalWebhookEndpoint) RotateSecret(ctx context.Context, endpointId string, endpointSecretRotateIn *OperationalWebhookEndpointSecretIn) error {
	return e.RotateSecretWithOptions(ctx, endpointId, endpointSecretRotateIn, nil)
}

func (e *OperationalWebhookEndpoint) RotateSecretWithOptions(ctx context.Context, endpointId string, endpointSecretRotateIn *OperationalWebhookEndpointSecretIn, options *PostOptions) error {
	req := e.api.WebhookEndpointAPI.RotateOperationalWebhookEndpointSecret(ctx, endpointId)
	req = req.OperationalWebhookEndpointSecretIn(*endpointSecretRotateIn)
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
