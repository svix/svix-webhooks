package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

type (
	AppPortalAccessIn  openapi.AppPortalAccessIn
	AppPortalAccessOut openapi.AppPortalAccessOut
	DashboardAccessOut openapi.DashboardAccessOut
)

func (a *Authentication) AppPortalAccess(appId string, appPortalAccessIn *AppPortalAccessIn) (*AppPortalAccessOut, error) {
	return a.AppPortalAccessWithOptions(appId, appPortalAccessIn, nil)
}

func (a *Authentication) AppPortalAccessWithOptions(appId string, appPortalAccessIn *AppPortalAccessIn, options *PostOptions) (*AppPortalAccessOut, error) {
	req := a.api.AuthenticationApi.GetAppPortalAccessApiV1AuthAppPortalAccessAppIdPost(context.Background(), appId)
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

func (a *Authentication) DashboardAccess(appId string) (*DashboardAccessOut, error) {
	return a.DashboardAccessWithOptions(appId, nil)
}

func (a *Authentication) DashboardAccessWithOptions(appId string, options *PostOptions) (*DashboardAccessOut, error) {
	req := a.api.AuthenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(context.Background(), appId)
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

func (a *Authentication) Logout() error {
	return a.LogoutWithOptions(nil)
}

func (a *Authentication) LogoutWithOptions(options *PostOptions) error {
	req := a.api.AuthenticationApi.LogoutApiV1AuthLogoutPost(context.Background())
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	res, err := req.Execute()
	return wrapError(err, res)
}
