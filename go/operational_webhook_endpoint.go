// this file is @generated (with some manual changes)
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
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

// List operational webhook endpoints.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) List(
	ctx context.Context,
	options *OperationalWebhookEndpointListOptions,
) (*ListResponseOperationalWebhookEndpointOut, error) {
	req := operationalWebhookEndpoint.api.WebhookEndpointAPI.ListOperationalWebhookEndpoints(
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

// Create an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Create(
	ctx context.Context,
	operationalWebhookEndpointIn *OperationalWebhookEndpointIn,
) (*OperationalWebhookEndpointOut, error) {
	return operationalWebhookEndpoint.CreateWithOptions(
		ctx,
		operationalWebhookEndpointIn,
		nil,
	)
}

// Create an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) CreateWithOptions(
	ctx context.Context,
	operationalWebhookEndpointIn *OperationalWebhookEndpointIn,
	options *PostOptions,
) (*OperationalWebhookEndpointOut, error) {
	req := operationalWebhookEndpoint.api.WebhookEndpointAPI.CreateOperationalWebhookEndpoint(
		ctx,
	).OperationalWebhookEndpointIn(*operationalWebhookEndpointIn)

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

// Get an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Get(
	ctx context.Context,
	endpointId string,
) (*OperationalWebhookEndpointOut, error) {
	req := operationalWebhookEndpoint.api.WebhookEndpointAPI.GetOperationalWebhookEndpoint(
		ctx,
		endpointId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Update an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Update(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointUpdate *OperationalWebhookEndpointUpdate,
) (*OperationalWebhookEndpointOut, error) {
	req := operationalWebhookEndpoint.api.WebhookEndpointAPI.UpdateOperationalWebhookEndpoint(
		ctx,
		endpointId,
	).OperationalWebhookEndpointUpdate(*operationalWebhookEndpointUpdate)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Delete an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Delete(
	ctx context.Context,
	endpointId string,
) error {
	req := operationalWebhookEndpoint.api.WebhookEndpointAPI.DeleteOperationalWebhookEndpoint(
		ctx,
		endpointId,
	)

	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *OperationalWebhookEndpoint) GetSecret(
	ctx context.Context,
	endpointId string,
) (*OperationalWebhookEndpointSecretOut, error) {
	req := e.api.WebhookEndpointAPI.GetOperationalWebhookEndpointSecret(ctx, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *OperationalWebhookEndpoint) RotateSecret(
	ctx context.Context,
	endpointId string,
	endpointSecretRotateIn *OperationalWebhookEndpointSecretIn,
) error {
	return e.RotateSecretWithOptions(ctx, endpointId, endpointSecretRotateIn, nil)
}

func (e *OperationalWebhookEndpoint) RotateSecretWithOptions(
	ctx context.Context,
	endpointId string,
	endpointSecretRotateIn *OperationalWebhookEndpointSecretIn,
	options *PostOptions,
) error {
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
