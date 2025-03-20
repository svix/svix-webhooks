// Package svix this file is @generated DO NOT EDIT
package models

// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event.
type MessageAttemptFailingEventData struct {
	AppId       string                   `json:"appId"`            // The Application's ID.
	AppUid      *string                  `json:"appUid,omitempty"` // The Application's UID.
	EndpointId  string                   `json:"endpointId"`       // The Endpoint's ID.
	LastAttempt MessageAttemptFailedData `json:"lastAttempt"`
	MsgEventId  *string                  `json:"msgEventId,omitempty"` // The Message's UID.
	MsgId       string                   `json:"msgId"`                // The Message's ID.
}
