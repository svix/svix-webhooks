// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Management struct {
	Authentication *ManagementAuthentication
}

func newManagement(client *internal.SvixHttpClient) *Management {
	return &Management{
		Authentication: newManagementAuthentication(client),
	}
}
