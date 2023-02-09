package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseEndpointOut   openapi.ListResponseEndpointOut
	EndpointIn                openapi.EndpointIn
	EndpointUpdate            openapi.EndpointUpdate
	EndpointOut               openapi.EndpointOut
	EndpointSecretOut         openapi.EndpointSecretOut
	EndpointSecretRotateIn    openapi.EndpointSecretRotateIn
	EndpointTransformationIn  openapi.EndpointTransformationIn
	RecoverIn                 openapi.RecoverIn
	ReplayIn                  openapi.ReplayIn
	EndpointHeadersIn         openapi.EndpointHeadersIn
	EndpointHeadersPatchIn    openapi.EndpointHeadersPatchIn
	EndpointHeadersOut        openapi.EndpointHeadersOut
	EndpointStats             openapi.EndpointStats
	EndpointTransformationOut openapi.EndpointTransformationOut
	Ordering                  openapi.Ordering
)

type Endpoint struct {
	api *openapi.APIClient
}

type EndpointListOptions struct {
	Iterator *string
	Limit    *int32
	Order    *Ordering
}

func (e *Endpoint) List(ctx context.Context, appId string, options *EndpointListOptions) (*ListResponseEndpointOut, error) {
	req := e.api.EndpointApi.ListEndpointsApiV1AppAppIdEndpointGet(ctx, appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Order != nil {
			req = req.Order(openapi.Ordering(*options.Order))
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseEndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Create(ctx context.Context, appId string, endpointIn *EndpointIn) (*EndpointOut, error) {
	return e.CreateWithOptions(ctx, appId, endpointIn, nil)
}

func (e *Endpoint) CreateWithOptions(ctx context.Context, appId string, endpointIn *EndpointIn, options *PostOptions) (*EndpointOut, error) {
	req := e.api.EndpointApi.CreateEndpointApiV1AppAppIdEndpointPost(ctx, appId)
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

func (e *Endpoint) Get(ctx context.Context, appId string, endpointId string) (*EndpointOut, error) {
	req := e.api.EndpointApi.GetEndpointApiV1AppAppIdEndpointEndpointIdGet(ctx, endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Update(ctx context.Context, appId string, endpointId string, endpointUpdate *EndpointUpdate) (*EndpointOut, error) {
	req := e.api.EndpointApi.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPut(ctx, endpointId, appId)
	req = req.EndpointUpdate(openapi.EndpointUpdate(*endpointUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointOut(out)
	return &ret, nil
}

func (e *Endpoint) Delete(ctx context.Context, appId string, endpointId string) error {
	req := e.api.EndpointApi.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(ctx, endpointId, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Endpoint) GetSecret(ctx context.Context, appId string, endpointId string) (*EndpointSecretOut, error) {
	req := e.api.EndpointApi.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(ctx, endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointSecretOut(out)
	return &ret, nil
}

func (e *Endpoint) RotateSecret(ctx context.Context, appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn) error {
	return e.RotateSecretWithOptions(ctx, appId, endpointId, endpointSecretRotateIn, nil)
}

func (e *Endpoint) RotateSecretWithOptions(ctx context.Context, appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn, options *PostOptions) error {
	req := e.api.EndpointApi.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePost(ctx, endpointId, appId)
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

func (e *Endpoint) Recover(ctx context.Context, appId string, endpointId string, recoverIn *RecoverIn) error {
	return e.RecoverWithOptions(ctx, appId, endpointId, recoverIn, nil)
}

func (e *Endpoint) RecoverWithOptions(ctx context.Context, appId string, endpointId string, recoverIn *RecoverIn, options *PostOptions) error {
	req := e.api.EndpointApi.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPost(ctx, appId, endpointId)
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

func (e *Endpoint) GetHeaders(ctx context.Context, appId string, endpointId string) (*EndpointHeadersOut, error) {
	req := e.api.EndpointApi.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(ctx, endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointHeadersOut(out)
	return &ret, nil
}

func (e *Endpoint) UpdateHeaders(ctx context.Context, appId string, endpointId string, endpointHeadersIn *EndpointHeadersIn) error {
	req := e.api.EndpointApi.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPut(ctx, appId, endpointId)
	req = req.EndpointHeadersIn(openapi.EndpointHeadersIn(*endpointHeadersIn))
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) PatchHeaders(ctx context.Context, appId string, endpointId string, endpointHeadersIn *EndpointHeadersPatchIn) error {
	req := e.api.EndpointApi.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatch(ctx, appId, endpointId)
	req = req.EndpointHeadersPatchIn(openapi.EndpointHeadersPatchIn(*endpointHeadersIn))
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) GetStats(ctx context.Context, appId string, endpointId string) (*EndpointStats, error) {
	req := e.api.EndpointApi.GetEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(ctx, endpointId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EndpointStats(out)
	return &ret, nil
}

func (e *Endpoint) ReplayMissing(ctx context.Context, appId string, endpointId string, replayIn *ReplayIn) error {
	return e.ReplayMissingWithOptions(ctx, appId, endpointId, replayIn, nil)
}

func (e *Endpoint) ReplayMissingWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn *ReplayIn,
	options *PostOptions,
) error {
	req := e.api.EndpointApi.ReplayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPost(ctx, appId, endpointId)
	req.ReplayIn(openapi.ReplayIn(*replayIn))
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

func (e *Endpoint) TransformationGet(ctx context.Context, appId string, endpointId string) (*EndpointTransformationOut, error) {
	req := e.api.EndpointApi.GetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGet(ctx, endpointId, appId)

	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	ret := EndpointTransformationOut(out)
	return &ret, nil
}

func (e *Endpoint) TransformatioPartialUpdate(ctx context.Context, appId string, endpointId string, transformation *EndpointTransformationIn) error {
	req := e.api.EndpointApi.SetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatch(ctx, appId, endpointId)
	req.EndpointTransformationIn(openapi.EndpointTransformationIn(*transformation))

	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}

	return nil
}
