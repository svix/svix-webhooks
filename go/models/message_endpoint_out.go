// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageEndpointOut struct {
	Id          string            `json:"id"` // The Endpoint's ID.
	Status      MessageStatus     `json:"status"`
	StatusText  MessageStatusText `json:"statusText"`
	NextAttempt *time.Time        `json:"nextAttempt,omitempty"`
	Url         string            `json:"url"`
	Description string            `json:"description"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16   `json:"throttleRate,omitempty"`
	Uid          *string   `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Disabled     *bool     `json:"disabled,omitempty"`
	EventTypes   []string  `json:"eventTypes,omitempty"`
	Channels     []string  `json:"channels,omitempty"` // List of message channels this endpoint listens to (omit for all).
	CreatedAt    time.Time `json:"createdAt"`
	UpdatedAt    time.Time `json:"updatedAt"`
}
