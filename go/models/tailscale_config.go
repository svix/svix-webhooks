// Package svix this file is @generated DO NOT EDIT
package models

type TailscaleConfig struct {
	Secret string `json:"secret"` // Shared secret for Tailscale Webhooks
	// Grace period (in seconds) for the timestamp.
	//
	// If not passed, timestamp age will not be checked.
	TimestampGraceSeconds *uint32 `json:"timestampGraceSeconds,omitempty"`
}
