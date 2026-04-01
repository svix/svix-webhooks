// Package svix this file is @generated DO NOT EDIT
package models

type EndpointIn struct {
	Channels    []string           `json:"channels,omitempty"` // List of message channels this endpoint listens to (omit for all).
	Description *string            `json:"description,omitempty"`
	Disabled    *bool              `json:"disabled,omitempty"`
	FilterTypes []string           `json:"filterTypes,omitempty"`
	Headers     *map[string]string `json:"headers,omitempty"`
	Metadata    *map[string]string `json:"metadata,omitempty"`
	RateLimit   *uint16            `json:"rateLimit,omitempty"` // Deprecated, use `throttleRate` instead.
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Secret *string `json:"secret,omitempty"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16 `json:"throttleRate,omitempty"`
	Uid          *string `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Url          string  `json:"url"`
	Version      *uint16 `json:"version,omitempty"`
}
