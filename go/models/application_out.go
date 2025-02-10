// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApplicationOut struct {
	CreatedAt time.Time `json:"createdAt"`
	// The app's ID
	Id        string            `json:"id"`
	Metadata  map[string]string `json:"metadata"`
	Name      string            `json:"name"`
	RateLimit *uint16           `json:"rateLimit,omitempty"`
	// The app's UID
	Uid       *string   `json:"uid,omitempty"`
	UpdatedAt time.Time `json:"updatedAt"`
}
