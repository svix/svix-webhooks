package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Endpoint struct {
	api *openapi.APIClient
}

type EndpointListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

type EndpointStatsOptions struct {
	// Filter the range to data starting from this date
	Since *time.Time
	// Filter the range to data ending by this date
	Until *time.Time
}

// List the application's endpoints.
func (endpoint *Endpoint) List(
	ctx context.Context,
	appId string,
	options *EndpointListOptions,
) (*ListResponseEndpointOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointList(
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

// Create a new endpoint for the application.
//
// When `secret` is `null` the secret is automatically generated (recommended).
func (endpoint *Endpoint) Create(
	ctx context.Context,
	appId string,
	endpointIn *EndpointIn,
) (*EndpointOut, error) {
	return endpoint.CreateWithOptions(
		ctx,
		appId,
		endpointIn,
		nil,
	)
}

// Create a new endpoint for the application.
//
// When `secret` is `null` the secret is automatically generated (recommended).
func (endpoint *Endpoint) CreateWithOptions(
	ctx context.Context,
	appId string,
	endpointIn *EndpointIn,
	options *PostOptions,
) (*EndpointOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointCreate(
		ctx,
		appId,
	).EndpointIn(*endpointIn)

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

// Get an endpoint.
func (endpoint *Endpoint) Get(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointGet(
		ctx,
		appId,
		endpointId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Update an endpoint.
func (endpoint *Endpoint) Update(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointUpdate *EndpointUpdate,
) (*EndpointOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointUpdate(
		ctx,
		appId,
		endpointId,
	).EndpointUpdate(*endpointUpdate)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

func (e *Endpoint) Delete(
	ctx context.Context,
	appId string,
	endpointId string,
) error {
	req := e.api.EndpointAPI.V1EndpointDelete(ctx, appId, endpointId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Endpoint) Patch(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointPatch *EndpointPatch,
) (*EndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointPatch(ctx, appId, endpointId)
	req = req.EndpointPatch(*endpointPatch)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) GetHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointHeadersOut, error) {
	req := e.api.EndpointAPI.V1EndpointGetHeaders(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) UpdateHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersIn *EndpointHeadersIn,
) error {
	req := e.api.EndpointAPI.V1EndpointUpdateHeaders(ctx, appId, endpointId)
	req = req.EndpointHeadersIn(*endpointHeadersIn)
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) PatchHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersIn *EndpointHeadersPatchIn,
) error {
	req := e.api.EndpointAPI.V1EndpointPatchHeaders(ctx, appId, endpointId)
	req = req.EndpointHeadersPatchIn(*endpointHeadersIn)
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) Recover(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn *RecoverIn,
) (*RecoverOut, error) {
	return e.RecoverWithOptions(ctx, appId, endpointId, recoverIn, nil)
}

func (e *Endpoint) RecoverWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn *RecoverIn,
	options *PostOptions,
) (*RecoverOut, error) {
	req := e.api.EndpointAPI.V1EndpointRecover(ctx, appId, endpointId)
	req = req.RecoverIn(*recoverIn)
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

// Replays messages to the endpoint.
//
// Only messages that were created after `since` will be sent.
// Messages that were previously sent to the endpoint are not resent.
func (endpoint *Endpoint) ReplayMissing(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn *ReplayIn,
) (*ReplayOut, error) {
	return endpoint.ReplayMissingWithOptions(
		ctx,
		appId,
		endpointId,
		replayIn,
		nil,
	)
}

// Replays messages to the endpoint.
//
// Only messages that were created after `since` will be sent.
// Messages that were previously sent to the endpoint are not resent.
func (endpoint *Endpoint) ReplayMissingWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn *ReplayIn,
	options *PostOptions,
) (*ReplayOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointReplayMissing(
		ctx,
		appId,
		endpointId,
	).ReplayIn(*replayIn)

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

func (e *Endpoint) GetSecret(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointSecretOut, error) {
	req := e.api.EndpointAPI.V1EndpointGetSecret(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) RotateSecret(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn *EndpointSecretRotateIn,
) error {
	return e.RotateSecretWithOptions(ctx, appId, endpointId, endpointSecretRotateIn, nil)
}

func (e *Endpoint) RotateSecretWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn *EndpointSecretRotateIn,
	options *PostOptions,
) error {
	req := e.api.EndpointAPI.V1EndpointRotateSecret(ctx, appId, endpointId)
	req = req.EndpointSecretRotateIn(*endpointSecretRotateIn)
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

func (e *Endpoint) SendExample(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
) (*MessageOut, error) {
	return e.SendExampleWithOptions(ctx, appId, endpointId, eventExampleIn, nil)
}

func (e *Endpoint) SendExampleWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
	options *PostOptions,
) (*MessageOut, error) {
	req := e.api.EndpointAPI.V1EndpointSendExample(ctx, appId, endpointId)
	req.EventExampleIn(*eventExampleIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) GetStats(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointStats, error) {
	return e.GetStatsWithOptions(ctx, appId, endpointId, EndpointStatsOptions{})
}

func (e *Endpoint) GetStatsWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	options EndpointStatsOptions,
) (*EndpointStats, error) {
	req := e.api.EndpointAPI.V1EndpointGetStats(ctx, appId, endpointId)
	if options.Since != nil {
		req = req.Since(*options.Since)
	}
	if options.Until != nil {
		req = req.Until(*options.Until)
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) TransformationGet(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointTransformationOut, error) {
	req := e.api.EndpointAPI.V1EndpointTransformationGet(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

func (e *Endpoint) TransformationPartialUpdate(
	ctx context.Context,
	appId string,
	endpointId string, transformation *EndpointTransformationIn,
) error {
	req := e.api.EndpointAPI.V1EndpointTransformationPartialUpdate(ctx, appId, endpointId)
	req = req.EndpointTransformationIn(*transformation)

	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}

	return nil
}
