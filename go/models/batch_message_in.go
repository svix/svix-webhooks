// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// A single message in a batch of bulk-created messages
//
// Practically identical to `MessageIn`, but without the `application` field
type BatchMessageIn struct {
	// Optional unique identifier for the message
	//
	// This should not contain the '.' character
	EventId   *string `json:"eventId,omitempty"`
	EventType string  `json:"eventType"` // The event type's name
	// JSON payload to send as the request body of the webhook.
	//
	// We also support sending non-JSON payloads. Please contact us for more information.
	//
	// The serialized byte-length of this value should be under 1MiB. Please contact us
	// if you need to send payloads large than 1MiB.
	Payload               map[string]any  `json:"payload"`
	Channels              []string        `json:"channels,omitempty"`              // List of free-form identifiers that endpoints can filter by
	Tags                  []string        `json:"tags,omitempty"`                  // List of free-form tags that can be filtered by when listing messages
	TransformationsParams *map[string]any `json:"transformationsParams,omitempty"` // Extra parameters to pass to Transformations (for future use)
	// The date and time at which the message will be delivered.
	//
	// Note that this time is best-effort-only. Must be at least one minute
	// and no more than 24 hours in the future.
	DeliverAt *time.Time `json:"deliverAt,omitempty"`
	// Optional number of days to retain the message payload. Defaults to 90.
	// Note that this is mutually exclusive with `payloadRetentionHours`.
	PayloadRetentionPeriod *int64 `json:"payloadRetentionPeriod,omitempty"`
	// Optional number of hours to retain the message payload. Note that
	// this is mutually exclusive with `payloadRetentionPeriod`.
	PayloadRetentionHours *int64 `json:"payloadRetentionHours,omitempty"`
}
