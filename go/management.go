// Package svix this file is @generated DO NOT EDIT
package svix

type Management struct {
	Authentication *ManagementAuthentication
}

func newManagement(client *SvixHttpClient) *Management {
	return &Management{
		Authentication: newManagementAuthentication(client),
	}
}
