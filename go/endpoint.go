// this file is @generated (with minor manual changes)
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

type EndpointGetStatsOptions struct {
	// Filter the range to data starting from this date.
	Since *time.Time
	// Filter the range to data ending by this date.
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

// Delete an endpoint.
func (endpoint *Endpoint) Delete(
	ctx context.Context,
	appId string,
	endpointId string,
) error {
	req := endpoint.api.EndpointAPI.V1EndpointDelete(
		ctx,
		appId,
		endpointId,
	)

	res, err := req.Execute()
	return wrapError(err, res)
}

// Partially update an endpoint.
func (endpoint *Endpoint) Patch(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointPatch *EndpointPatch,
) (*EndpointOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointPatch(
		ctx,
		appId,
		endpointId,
	).EndpointPatch(*endpointPatch)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Get the additional headers to be sent with the webhook.
func (endpoint *Endpoint) GetHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointHeadersOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointGetHeaders(
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

// Set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) UpdateHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersIn *EndpointHeadersIn,
) error {
	req := endpoint.api.EndpointAPI.V1EndpointUpdateHeaders(
		ctx,
		appId,
		endpointId,
	).EndpointHeadersIn(*endpointHeadersIn)

	res, err := req.Execute()
	return wrapError(err, res)
}

// Partially set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) PatchHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersPatchIn *EndpointHeadersPatchIn,
) error {
	req := endpoint.api.EndpointAPI.V1EndpointPatchHeaders(
		ctx,
		appId,
		endpointId,
	).EndpointHeadersPatchIn(*endpointHeadersPatchIn)

	res, err := req.Execute()
	return wrapError(err, res)
}

// Resend all failed messages since a given time.
//
// Messages that were sent successfully, even if failed initially, are not resent.
func (endpoint *Endpoint) Recover(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn *RecoverIn,
) (*RecoverOut, error) {
	return endpoint.RecoverWithOptions(
		ctx,
		appId,
		endpointId,
		recoverIn,
		nil,
	)
}

// Resend all failed messages since a given time.
//
// Messages that were sent successfully, even if failed initially, are not resent.
func (endpoint *Endpoint) RecoverWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn *RecoverIn,
	options *PostOptions,
) (*RecoverOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointRecover(
		ctx,
		appId,
		endpointId,
	).RecoverIn(*recoverIn)

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

// Get the endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (endpoint *Endpoint) GetSecret(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointSecretOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointGetSecret(
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

// Rotates the endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (endpoint *Endpoint) RotateSecret(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn *EndpointSecretRotateIn,
) error {
	return endpoint.RotateSecretWithOptions(
		ctx,
		appId,
		endpointId,
		endpointSecretRotateIn,
		nil,
	)
}

// Rotates the endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (endpoint *Endpoint) RotateSecretWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn *EndpointSecretRotateIn,
	options *PostOptions,
) error {
	req := endpoint.api.EndpointAPI.V1EndpointRotateSecret(
		ctx,
		appId,
		endpointId,
	).EndpointSecretRotateIn(*endpointSecretRotateIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	res, err := req.Execute()
	return wrapError(err, res)
}

// Send an example message for an event.
func (endpoint *Endpoint) SendExample(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
) (*MessageOut, error) {
	return endpoint.SendExampleWithOptions(
		ctx,
		appId,
		endpointId,
		eventExampleIn,
		nil,
	)
}

// Send an example message for an event.
func (endpoint *Endpoint) SendExampleWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
	options *PostOptions,
) (*MessageOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointSendExample(
		ctx,
		appId,
		endpointId,
	).EventExampleIn(*eventExampleIn)

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

// Get basic statistics for the endpoint.
func (endpoint *Endpoint) GetStats(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointStats, error) {
	return endpoint.GetStatsWithOptions(ctx, appId, endpointId, EndpointStatsOptions{})
}

// Get basic statistics for the endpoint.
func (endpoint *Endpoint) GetStatsWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	options EndpointStatsOptions,
) (*EndpointStats, error) {
	req := endpoint.api.EndpointAPI.V1EndpointGetStats(
		ctx,
		appId,
		endpointId,
	)

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

// Get the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationGet(
	ctx context.Context,
	appId string,
	endpointId string,
) (*EndpointTransformationOut, error) {
	req := endpoint.api.EndpointAPI.V1EndpointTransformationGet(
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

// Set or unset the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationPartialUpdate(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointTransformationIn *EndpointTransformationIn,
) error {
	req := endpoint.api.EndpointAPI.V1EndpointTransformationPartialUpdate(
		ctx,
		appId,
		endpointId,
	).EndpointTransformationIn(*endpointTransformationIn)

	res, err := req.Execute()
	return wrapError(err, res)
}
