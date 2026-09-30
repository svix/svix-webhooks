// Package svix this file is @generated DO NOT EDIT
package models

type CreateAutoConfigSubscriptionIn struct {
	// The set of feature flags the created token will have access to.
	//
	// When omitted or empty, the token inherits the calling token's feature flags.
	// When set, these flags are used instead. An application token may only grant a subset of its own flags.
	FeatureFlags []string `json:"featureFlags,omitempty"`
}
