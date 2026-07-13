// Package svix this file is @generated DO NOT EDIT
package models

type OperationalWebhookEndpointIn struct {
	Description *string `json:"description,omitempty"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16  `json:"throttleRate,omitempty"`
	Uid          *string  `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Url          string   `json:"url"`
	Disabled     *bool    `json:"disabled,omitempty"`
	FilterTypes  []string `json:"filterTypes,omitempty"`
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Secret   *string            `json:"secret,omitempty"`
	Metadata *map[string]string `json:"metadata,omitempty"`
}
