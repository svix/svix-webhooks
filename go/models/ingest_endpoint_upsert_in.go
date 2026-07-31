// Package svix this file is @generated DO NOT EDIT
package models

type IngestEndpointUpsertIn struct {
	Url         string  `json:"url"`
	Description *string `json:"description,omitempty"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16            `json:"throttleRate,omitempty"`
	Uid          *string            `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Disabled     *bool              `json:"disabled,omitempty"`
	Metadata     *map[string]string `json:"metadata,omitempty"`
}
