package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

type DashboardAccessOut openapi.DashboardAccessOut

func (a *Authentication) DashboardAccess(appId string) (*DashboardAccessOut, error) {
	req := a.api.AuthenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(context.Background(), appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := DashboardAccessOut(out)
	return &ret, nil
}

func (a *Authentication) Logout() error {
	req := a.api.AuthenticationApi.LogoutApiV1AuthLogoutPost(context.Background())
	res, err := req.Execute()
	return wrapError(err, res)
}
