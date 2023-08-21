package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

type (
	AppPortalAccessIn  = openapi.AppPortalAccessIn
	AppPortalAccessOut = openapi.AppPortalAccessOut
	DashboardAccessOut = openapi.DashboardAccessOut
)

func (a *Authentication) AppPortalAccess(ctx context.Context, appId string, appPortalAccessIn *AppPortalAccessIn) (*AppPortalAccessOut, error) {
	return a.AppPortalAccessWithOptions(ctx, appId, appPortalAccessIn, nil)
}

func (a *Authentication) AppPortalAccessWithOptions(ctx context.Context, appId string, appPortalAccessIn *AppPortalAccessIn, options *PostOptions) (*AppPortalAccessOut, error) {
	req := a.api.AuthenticationApi.V1AuthenticationAppPortalAccess(ctx, appId)
	req = req.AppPortalAccessIn(openapi.AppPortalAccessIn(*appPortalAccessIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := AppPortalAccessOut(out)
	return &ret, nil
}

func (a *Authentication) DashboardAccess(ctx context.Context, appId string) (*DashboardAccessOut, error) {
	return a.DashboardAccessWithOptions(ctx, appId, nil)
}

func (a *Authentication) DashboardAccessWithOptions(ctx context.Context, appId string, options *PostOptions) (*DashboardAccessOut, error) {
	req := a.api.AuthenticationApi.V1AuthenticationDashboardAccess(ctx, appId)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := DashboardAccessOut(out)
	return &ret, nil
}

func (a *Authentication) Logout(ctx context.Context) error {
	return a.LogoutWithOptions(ctx, nil)
}

func (a *Authentication) LogoutWithOptions(ctx context.Context, options *PostOptions) error {
	req := a.api.AuthenticationApi.V1AuthenticationLogout(ctx)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	res, err := req.Execute()
	return wrapError(err, res)
}
