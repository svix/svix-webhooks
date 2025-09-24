// Package svix this file is @generated DO NOT EDIT
package models

// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed four times as a "ingest.message.attempt.failing" event, or after it's recovered as a "ingest.message.attempt.recovered" event.
type IngestMessageAttemptExhaustedEventData struct {
	EndpointId  string                   `json:"endpointId"` // The Endpoint's ID.
	LastAttempt MessageAttemptFailedData `json:"lastAttempt"`
	MsgEventId  *string                  `json:"msgEventId,omitempty"` // The Message's UID.
	MsgId       string                   `json:"msgId"`                // The Message's ID.
	SourceId    string                   `json:"sourceId"`             // The Source's ID.
}
