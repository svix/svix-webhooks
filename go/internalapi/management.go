// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Management struct {
	Authentication      *ManagementAuthentication
	Environment         *ManagementEnvironment
	EnvironmentSettings *ManagementEnvironmentSettings
}

func newManagement(client *internal.SvixHttpClient) *Management {
	return &Management{
		Authentication:      newManagementAuthentication(client),
		Environment:         newManagementEnvironment(client),
		EnvironmentSettings: newManagementEnvironmentSettings(client),
	}
}
