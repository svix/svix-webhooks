// non-generated extension for application.go
package svix

import (
	"context"
)

func (a *Application) GetOrCreate(
	ctx context.Context,
	applicationIn *ApplicationIn,
) (*ApplicationOut, error) {
	return a.GetOrCreateWithOptions(ctx, applicationIn, nil)
}

func (a *Application) GetOrCreateWithOptions(
	ctx context.Context,
	applicationIn *ApplicationIn,
	options *PostOptions,
) (*ApplicationOut, error) {
	req := a.api.ApplicationAPI.V1ApplicationCreate(ctx)
	req = req.ApplicationIn(*applicationIn)
	req = req.GetIfExists(true)
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
