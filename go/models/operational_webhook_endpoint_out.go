// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type OperationalWebhookEndpointOut struct {
	Id          string `json:"id"` // The Endpoint's ID.
	Url         string `json:"url"`
	Description string `json:"description"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16           `json:"throttleRate,omitempty"`
	Uid          *string           `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Disabled     *bool             `json:"disabled,omitempty"`
	EventTypes   []string          `json:"eventTypes,omitempty"`
	CreatedAt    time.Time         `json:"createdAt"`
	UpdatedAt    time.Time         `json:"updatedAt"`
	Metadata     map[string]string `json:"metadata"`
}
