// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Management struct {
	client *internal.SvixHttpClient
}

func newManagement(client *internal.SvixHttpClient) Management {
	return Management{client}
}

func (management Management) Authentication() ManagementAuthentication {
	return newManagementAuthentication(management.client)
}
func (management Management) Environment() ManagementEnvironment {
	return newManagementEnvironment(management.client)
}
func (management Management) EnvironmentSettings() ManagementEnvironmentSettings {
	return newManagementEnvironmentSettings(management.client)
}
