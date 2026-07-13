// Package svix this file is @generated DO NOT EDIT
package models

type SinkInCommon struct {
	Description *string `json:"description,omitempty"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16 `json:"throttleRate,omitempty"`
	Uid          *string `json:"uid,omitempty"` // Optional unique identifier for the sink.
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Secret      *string            `json:"secret,omitempty"`
	Disabled    *bool              `json:"disabled,omitempty"`
	FilterTypes []string           `json:"filterTypes,omitempty"`
	Channels    []string           `json:"channels,omitempty"` // List of message channels this sink listens to (omit for all).
	Metadata    *map[string]string `json:"metadata,omitempty"`
}
