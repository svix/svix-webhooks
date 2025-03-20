// Package svix this file is @generated DO NOT EDIT
package models

// Sent when a message delivery has failed (all of the retry attempts have been exhausted).
type MessageAttemptExhaustedEvent struct {
	Data MessageAttemptExhaustedEventData `json:"data"`
	Type string                           `json:"type"`
}
