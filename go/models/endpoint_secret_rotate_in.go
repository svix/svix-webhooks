// Package svix this file is @generated DO NOT EDIT
package models

type EndpointSecretRotateIn struct {
	// How long the old secret will be valid for, in seconds.
	//
	// Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours.
	GracePeriodSeconds *uint32 `json:"gracePeriodSeconds,omitempty"`
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Key *string `json:"key,omitempty"`
}
