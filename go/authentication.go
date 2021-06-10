package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

type DashboardAccessOut openapi.DashboardAccessOut

func (a *Authentication) DashboardAccess(appId string) (*DashboardAccessOut, error) {
	req := a.api.AuthenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(context.Background(), appId)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := DashboardAccessOut(out)
	return &ret, nil
}

func (a *Authentication) Logout() error {
	req := a.api.AuthenticationApi.LogoutApiV1AuthLogoutPost(context.Background())
	_, err := req.Execute()
	return err
}
