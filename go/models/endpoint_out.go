// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EndpointOut struct {
	Id          string            `json:"id"` // The Endpoint's ID.
	Metadata    map[string]string `json:"metadata"`
	Description string            `json:"description"` // An example endpoint name.
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16   `json:"throttleRate,omitempty"`
	Uid          *string   `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Url          string    `json:"url"`
	Disabled     *bool     `json:"disabled,omitempty"`
	FilterTypes  []string  `json:"filterTypes,omitempty"`
	Channels     []string  `json:"channels,omitempty"` // List of message channels this endpoint listens to (omit for all).
	CreatedAt    time.Time `json:"createdAt"`
	UpdatedAt    time.Time `json:"updatedAt"`
}
