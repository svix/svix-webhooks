// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Authentication struct {
	client *internal.SvixHttpClient
}

func newAuthentication(client *internal.SvixHttpClient) *Authentication {
	return &Authentication{
		client: client,
	}
}

type AuthenticationAppPortalAccessOptions struct {
	IdempotencyKey *string
}

type AuthenticationExpireAllOptions struct {
	IdempotencyKey *string
}

type AuthenticationLogoutOptions struct {
	IdempotencyKey *string
}

type AuthenticationStreamPortalAccessOptions struct {
	IdempotencyKey *string
}

type AuthenticationRotateStreamPollerTokenOptions struct {
	IdempotencyKey *string
}

// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) AppPortalAccess(
	ctx context.Context,
	appId string,
	appPortalAccessIn models.AppPortalAccessIn,
	o *AuthenticationAppPortalAccessOptions,
) (*models.AppPortalAccessOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.AppPortalAccessIn, models.AppPortalAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/app-portal-access/{app_id}",
		pathMap,
		nil,
		headerMap,
		&appPortalAccessIn,
	)
}

// Expire all of the tokens associated with a specific application.
func (authentication *Authentication) ExpireAll(
	ctx context.Context,
	appId string,
	applicationTokenExpireIn models.ApplicationTokenExpireIn,
	o *AuthenticationExpireAllOptions,
) error {
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[models.ApplicationTokenExpireIn, any](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/app/{app_id}/expire-all",
		pathMap,
		nil,
		headerMap,
		&applicationTokenExpireIn,
	)
	return err
}

// Deprecated: Please use `AppPortalAccess` instead.
func (authentication *Authentication) DashboardAccess(
	ctx context.Context,
	appId string,
	o *AuthenticationDashboardAccessOptions,
) (*models.DashboardAccessOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.DashboardAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/dashboard-access/{app_id}",
		pathMap,
		nil,
		headerMap,
		nil,
	)
}

// Logout an app token.
//
// Trying to log out other tokens will fail.
func (authentication *Authentication) Logout(
	ctx context.Context,
	o *AuthenticationLogoutOptions,
) error {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/logout",
		nil,
		nil,
		headerMap,
		nil,
	)
	return err
}

// Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
func (authentication *Authentication) StreamPortalAccess(
	ctx context.Context,
	streamId string,
	streamPortalAccessIn models.StreamPortalAccessIn,
	o *AuthenticationStreamPortalAccessOptions,
) (*models.AppPortalAccessOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.StreamPortalAccessIn, models.AppPortalAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/stream-portal-access/{stream_id}",
		pathMap,
		nil,
		headerMap,
		&streamPortalAccessIn,
	)
}

// Get the current auth token for the stream poller.
func (authentication *Authentication) GetStreamPollerToken(
	ctx context.Context,
	streamId string,
	sinkId string,
) (*models.ApiTokenOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	return internal.ExecuteRequest[any, models.ApiTokenOut](
		ctx,
		authentication.client,
		"GET",
		"/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Create a new auth token for the stream poller API.
func (authentication *Authentication) RotateStreamPollerToken(
	ctx context.Context,
	streamId string,
	sinkId string,
	rotatePollerTokenIn models.RotatePollerTokenIn,
	o *AuthenticationRotateStreamPollerTokenOptions,
) (*models.ApiTokenOut, error) {
	pathMap := map[string]string{
		"stream_id": streamId,
		"sink_id":   sinkId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.RotatePollerTokenIn, models.ApiTokenOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token/rotate",
		pathMap,
		nil,
		headerMap,
		&rotatePollerTokenIn,
	)
}
