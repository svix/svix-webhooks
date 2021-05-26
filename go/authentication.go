package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

type DashboardAccessOut openapi.DashboardAccessOut

func (a *Authentication) DashboardAccess(appID string) (*DashboardAccessOut, error) {
	req := a.api.AuthenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(context.Background(), appID)
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
