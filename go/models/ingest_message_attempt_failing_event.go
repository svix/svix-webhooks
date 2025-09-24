// Package svix this file is @generated DO NOT EDIT
package models

// Sent after a message has been failing for a few times.
// It's sent on the fourth failure. It complements `ingest.message.attempt.exhausted` which is sent after the last failure.
type IngestMessageAttemptFailingEvent struct {
	Data IngestMessageAttemptFailingEventData `json:"data"`
	Type string                               `json:"type"`
}
