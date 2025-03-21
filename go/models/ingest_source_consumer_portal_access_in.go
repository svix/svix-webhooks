// Package svix this file is @generated DO NOT EDIT
package models

type IngestSourceConsumerPortalAccessIn struct {
	// How long the token will be valid for, in seconds.
	//
	// Valid values are between 1 hour and 7 days. The default is 7 days.
	Expiry   *uint64 `json:"expiry,omitempty"`
	ReadOnly *bool   `json:"readOnly,omitempty"` // Whether the app portal should be in read-only mode.
}
