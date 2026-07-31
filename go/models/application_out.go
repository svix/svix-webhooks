// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApplicationOut struct {
	Uid  *string `json:"uid,omitempty"` // Optional unique identifier for the application.
	Name string  `json:"name"`          // Application name for human consumption.
	// Maximum messages per second to send to this application.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16           `json:"throttleRate,omitempty"`
	Id           string            `json:"id"` // The Application's ID.
	CreatedAt    time.Time         `json:"createdAt"`
	UpdatedAt    time.Time         `json:"updatedAt"`
	Metadata     map[string]string `json:"metadata"`
}
