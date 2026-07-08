// Package svix this file is @generated DO NOT EDIT
package models

type ApplicationIn struct {
	Metadata *map[string]string `json:"metadata,omitempty"`
	Name     string             `json:"name"` // Application name for human consumption.
	// Maximum messages per second to send to this application.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16 `json:"throttleRate,omitempty"`
	Uid          *string `json:"uid,omitempty"` // Optional unique identifier for the application.
}
