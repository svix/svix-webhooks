// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApplicationOut struct {
	CreatedAt time.Time         `json:"createdAt"`
	Id        string            `json:"id"` // The Application's ID.
	Metadata  map[string]string `json:"metadata"`
	Name      string            `json:"name"`                // Application name for human consumption.
	RateLimit *uint16           `json:"rateLimit,omitempty"` // Deprecated, use `throttleRate` instead.
	// Maximum messages per second to send to this application.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate *uint16   `json:"throttleRate,omitempty"`
	Uid          *string   `json:"uid,omitempty"` // Optional unique identifier for the application.
	UpdatedAt    time.Time `json:"updatedAt"`
}
