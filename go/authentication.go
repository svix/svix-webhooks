// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/models"
)

type Authentication struct {
	client *SvixHttpClient
}

type AuthenticationAppPortalAccessOptions struct {
	IdempotencyKey *string
}

type AuthenticationExpireAllOptions struct {
	IdempotencyKey *string
}

type AuthenticationDashboardAccessOptions struct {
	IdempotencyKey *string
}

type AuthenticationLogoutOptions struct {
	IdempotencyKey *string
}

// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) AppPortalAccess(
	ctx context.Context,
	appId string,
	appPortalAccessIn *models.AppPortalAccessIn,
	o *AuthenticationAppPortalAccessOptions,
) (*models.AppPortalAccessOut, error) {
	if appPortalAccessIn == nil {
		return nil, fmt.Errorf("Authentication.AppPortalAccess(), appPortalAccessIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err = json.Marshal(appPortalAccessIn)
	if err != nil {
		return nil, err
	}
	ret, err := executeRequest[models.AppPortalAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/app-portal-access/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Expire all of the tokens associated with a specific application.
func (authentication *Authentication) ExpireAll(
	ctx context.Context,
	appId string,
	applicationTokenExpireIn *models.ApplicationTokenExpireIn,
	o *AuthenticationExpireAllOptions,
) error {
	if applicationTokenExpireIn == nil {
		return fmt.Errorf("Authentication.ExpireAll(), applicationTokenExpireIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	jsonBody, err = json.Marshal(applicationTokenExpireIn)
	if err != nil {
		return err
	}
	_, err = executeRequest[any](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/app/{app_id}/expire-all",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return err
	}
	return nil
}

// DEPRECATED: Please use `app-portal-access` instead.
//
// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
func (authentication *Authentication) DashboardAccess(
	ctx context.Context,
	appId string,
	o *AuthenticationDashboardAccessOptions,
) (*models.DashboardAccessOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[models.DashboardAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/dashboard-access/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Logout an app token.
//
// Trying to log out other tokens will fail.
func (authentication *Authentication) Logout(
	ctx context.Context,
	o *AuthenticationLogoutOptions,
) error {
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = executeRequest[any](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/logout",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return err
	}
	return nil
}
