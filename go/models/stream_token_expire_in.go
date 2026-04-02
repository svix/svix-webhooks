// Package svix this file is @generated DO NOT EDIT
package models

type StreamTokenExpireIn struct {
	Expiry *int64 `json:"expiry,omitempty"` // How many seconds until the old key is expired.
	// An optional list of session ids.
	//
	// If any session ids are specified, only Stream tokens created with that session id will be expired.
	SessionIds []string `json:"sessionIds,omitempty"`
}
