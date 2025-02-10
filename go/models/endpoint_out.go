// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EndpointOut struct {
	// List of message channels this endpoint listens to (omit for all).
	Channels  []string  `json:"channels,omitempty"`
	CreatedAt time.Time `json:"createdAt"`
	// An example endpoint name.
	Description string   `json:"description"`
	Disabled    *bool    `json:"disabled,omitempty"`
	FilterTypes []string `json:"filterTypes,omitempty"`
	// The ep's ID
	Id        string            `json:"id"`
	Metadata  map[string]string `json:"metadata"`
	RateLimit *uint16           `json:"rateLimit,omitempty"`
	// Optional unique identifier for the endpoint.
	Uid       *string   `json:"uid,omitempty"`
	UpdatedAt time.Time `json:"updatedAt"`
	Url       string    `json:"url"`
	Version   int32     `json:"version"`
}
