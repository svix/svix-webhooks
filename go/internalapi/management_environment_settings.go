// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type ManagementEnvironmentSettings struct {
	client *internal.SvixHttpClient
}

func newManagementEnvironmentSettings(client *internal.SvixHttpClient) *ManagementEnvironmentSettings {
	return &ManagementEnvironmentSettings{
		client: client,
	}
}

// Get the environments's settings
func (managementEnvironmentSettings *ManagementEnvironmentSettings) Get(
	ctx context.Context,
) (*models.SettingsInternalOut, error) {
	return internal.ExecuteRequest[any, models.SettingsInternalOut](
		ctx,
		managementEnvironmentSettings.client,
		"GET",
		"/api/v1/management/environment-settings",
		nil,
		nil,
		nil,
		nil,
	)
}

// Update the environment's settings
func (managementEnvironmentSettings *ManagementEnvironmentSettings) Update(
	ctx context.Context,
	settingsInternalIn models.SettingsInternalIn,
) (*models.SettingsInternalOut, error) {
	return internal.ExecuteRequest[models.SettingsInternalIn, models.SettingsInternalOut](
		ctx,
		managementEnvironmentSettings.client,
		"PUT",
		"/api/v1/management/environment-settings",
		nil,
		nil,
		nil,
		&settingsInternalIn,
	)
}
