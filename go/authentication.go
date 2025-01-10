// this file is @generated
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Authentication struct {
	api *openapi.APIClient
}

// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) AppPortalAccess(
	ctx context.Context,
	appId string,
	appPortalAccessIn *AppPortalAccessIn,
) (*AppPortalAccessOut, error) {
	return authentication.AppPortalAccessWithOptions(
		ctx,
		appId,
		appPortalAccessIn,
		nil,
	)
}

// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) AppPortalAccessWithOptions(
	ctx context.Context,
	appId string,
	appPortalAccessIn *AppPortalAccessIn,
	options *PostOptions,
) (*AppPortalAccessOut, error) {
	req := authentication.api.AuthenticationAPI.V1AuthenticationAppPortalAccess(
		ctx,
		appId,
	).AppPortalAccessIn(*appPortalAccessIn)

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

// Expire all of the tokens associated with a specific application.
func (authentication *Authentication) ExpireAll(
	ctx context.Context,
	appId string,
	applicationTokenExpireIn *ApplicationTokenExpireIn,
) error {
	return authentication.ExpireAllWithOptions(
		ctx,
		appId,
		applicationTokenExpireIn,
		nil,
	)
}

// Expire all of the tokens associated with a specific application.
func (authentication *Authentication) ExpireAllWithOptions(
	ctx context.Context,
	appId string,
	applicationTokenExpireIn *ApplicationTokenExpireIn,
	options *PostOptions,
) error {
	req := authentication.api.AuthenticationAPI.V1AuthenticationExpireAll(
		ctx,
		appId,
	).ApplicationTokenExpireIn(*applicationTokenExpireIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	res, err := req.Execute()
	return wrapError(err, res)
}

// DEPRECATED: Please use `app-portal-access` instead.
//
// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) DashboardAccess(
	ctx context.Context,
	appId string,
) (*DashboardAccessOut, error) {
	return authentication.DashboardAccessWithOptions(
		ctx,
		appId,
		nil,
	)
}

// DEPRECATED: Please use `app-portal-access` instead.
//
// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) DashboardAccessWithOptions(
	ctx context.Context,
	appId string,
	options *PostOptions,
) (*DashboardAccessOut, error) {
	req := authentication.api.AuthenticationAPI.V1AuthenticationDashboardAccess(
		ctx,
		appId,
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

// Logout an app token.
//
// Trying to log out other tokens will fail.
func (authentication *Authentication) Logout(
	ctx context.Context,
) error {
	return authentication.LogoutWithOptions(
		ctx,
		nil,
	)
}

// Logout an app token.
//
// Trying to log out other tokens will fail.
func (authentication *Authentication) LogoutWithOptions(
	ctx context.Context,
	options *PostOptions,
) error {
	req := authentication.api.AuthenticationAPI.V1AuthenticationLogout(
		ctx,
	)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	res, err := req.Execute()
	return wrapError(err, res)
}
