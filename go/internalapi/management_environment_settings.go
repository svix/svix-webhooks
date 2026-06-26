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
) (*models.SettingsInternalUpdateOut, error) {
	return internal.ExecuteRequest[models.SettingsInternalIn, models.SettingsInternalUpdateOut](
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

// Patch environment settings
func (managementEnvironmentSettings *ManagementEnvironmentSettings) Patch(
	ctx context.Context,
	settingsInternalPatch models.SettingsInternalPatch,
) (*models.SettingsInternalOut, error) {
	return internal.ExecuteRequest[models.SettingsInternalPatch, models.SettingsInternalOut](
		ctx,
		managementEnvironmentSettings.client,
		"PATCH",
		"/api/v1/management/environment-settings",
		nil,
		nil,
		nil,
		&settingsInternalPatch,
	)
}

// Get customer otel config.
func (managementEnvironmentSettings *ManagementEnvironmentSettings) GetOtelConfig(
	ctx context.Context,
) (*models.OtelConfigOut, error) {
	return internal.ExecuteRequest[any, models.OtelConfigOut](
		ctx,
		managementEnvironmentSettings.client,
		"GET",
		"/api/v1/management/environment-settings/customer-otel",
		nil,
		nil,
		nil,
		nil,
	)
}

// Update customer otel config.
func (managementEnvironmentSettings *ManagementEnvironmentSettings) UpdateOtelConfig(
	ctx context.Context,
	otelConfig models.OtelConfig,
) error {
	_, err := internal.ExecuteRequest[models.OtelConfig, any](
		ctx,
		managementEnvironmentSettings.client,
		"PUT",
		"/api/v1/management/environment-settings/customer-otel",
		nil,
		nil,
		nil,
		&otelConfig,
	)
	return err
}

// Delete customer otel config.
func (managementEnvironmentSettings *ManagementEnvironmentSettings) DeleteOtelConfig(
	ctx context.Context,
) error {
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		managementEnvironmentSettings.client,
		"DELETE",
		"/api/v1/management/environment-settings/customer-otel",
		nil,
		nil,
		nil,
		nil,
	)
	return err
}
