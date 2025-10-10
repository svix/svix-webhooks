// Package svix this file is @generated DO NOT EDIT
package models

type StreamPortalAccessIn struct {
	// How long the token will be valid for, in seconds.
	//
	// Valid values are between 1 hour and 7 days. The default is 7 days.
	Expiry       *uint64  `json:"expiry,omitempty"`
	FeatureFlags []string `json:"featureFlags,omitempty"` // The set of feature flags the created token will have access to.
	// An optional session ID to attach to the token.
	//
	// When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
	SessionId *string `json:"sessionId,omitempty"`
}
