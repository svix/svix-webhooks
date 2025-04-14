// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Management struct {
	Environment         *ManagementEnvironment
	EnvironmentSettings *ManagementEnvironmentSettings
}

func newManagement(client *internal.SvixHttpClient) *Management {
	return &Management{
		Environment:         newManagementEnvironment(client),
		EnvironmentSettings: newManagementEnvironmentSettings(client),
	}
}
