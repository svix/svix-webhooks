// Package svix this file is @generated DO NOT EDIT
package models

// Sent when a message delivery has failed (all of the retry attempts have been exhausted).
type IngestMessageAttemptExhaustedEvent struct {
	Data IngestMessageAttemptExhaustedEventData `json:"data"`
	Type string                                 `json:"type"`
}
