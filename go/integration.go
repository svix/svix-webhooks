// this file is @generated
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Integration struct {
	api *openapi.APIClient
}

type IntegrationListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

// List the application's integrations.
func (integration *Integration) List(
	ctx context.Context,
	appId string,
	options *IntegrationListOptions,
) (*ListResponseIntegrationOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationList(
		ctx,
		appId,
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

// Create an integration.
func (integration *Integration) Create(
	ctx context.Context,
	appId string,
	integrationIn *IntegrationIn,
) (*IntegrationOut, error) {
	return integration.CreateWithOptions(
		ctx,
		appId,
		integrationIn,
		nil,
	)
}

// Create an integration.
func (integration *Integration) CreateWithOptions(
	ctx context.Context,
	appId string,
	integrationIn *IntegrationIn,
	options *PostOptions,
) (*IntegrationOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationCreate(
		ctx,
		appId,
	).IntegrationIn(*integrationIn)

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

// Get an integration.
func (integration *Integration) Get(
	ctx context.Context,
	appId string,
	integId string,
) (*IntegrationOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationGet(
		ctx,
		appId,
		integId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Update an integration.
func (integration *Integration) Update(
	ctx context.Context,
	appId string,
	integId string,
	integrationUpdate *IntegrationUpdate,
) (*IntegrationOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationUpdate(
		ctx,
		appId,
		integId,
	).IntegrationUpdate(*integrationUpdate)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Delete an integration.
func (integration *Integration) Delete(
	ctx context.Context,
	appId string,
	integId string,
) error {
	req := integration.api.IntegrationAPI.V1IntegrationDelete(
		ctx,
		appId,
		integId,
	)

	res, err := req.Execute()
	return wrapError(err, res)
}

// Get an integration's key.
func (integration *Integration) GetKey(
	ctx context.Context,
	appId string,
	integId string,
) (*IntegrationKeyOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationGetKey(
		ctx,
		appId,
		integId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Rotate the integration's key. The previous key will be immediately revoked.
func (integration *Integration) RotateKey(
	ctx context.Context,
	appId string,
	integId string,
) (*IntegrationKeyOut, error) {
	return integration.RotateKeyWithOptions(
		ctx,
		appId,
		integId,
		nil,
	)
}

// Rotate the integration's key. The previous key will be immediately revoked.
func (integration *Integration) RotateKeyWithOptions(
	ctx context.Context,
	appId string,
	integId string,
	options *PostOptions,
) (*IntegrationKeyOut, error) {
	req := integration.api.IntegrationAPI.V1IntegrationRotateKey(
		ctx,
		appId,
		integId,
	)

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
