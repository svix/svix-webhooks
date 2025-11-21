// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageAttemptLog struct {
	AppId              string            `json:"appId"`            // The Application's ID.
	AppUid             *string           `json:"appUid,omitempty"` // The Application's UID.
	AttemptCount       uint16            `json:"attemptCount"`
	AttemptEnd         time.Time         `json:"attemptEnd"`
	AttemptId          string            `json:"attemptId"` // The MessageAttempt's ID.
	AttemptStart       time.Time         `json:"attemptStart"`
	EndpointId         string            `json:"endpointId"`          // The Endpoint's ID.
	EventType          *string           `json:"eventType,omitempty"` // The event type's name
	HttpTimes          *HttpAttemptTimes `json:"httpTimes,omitempty"`
	MsgCreated         time.Time         `json:"msgCreated"`
	MsgEventId         *string           `json:"msgEventId,omitempty"` // The Message's UID.
	MsgId              string            `json:"msgId"`                // The Message's ID.
	OrgId              string            `json:"orgId"`                // The Environment's ID.
	ResponseStatusCode int16             `json:"responseStatusCode"`
	Status             MessageStatus     `json:"status"`
}
