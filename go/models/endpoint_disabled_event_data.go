// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call.
type EndpointDisabledEventData struct {
	AppId       string                   `json:"appId"`                 // The Application's ID.
	AppUid      *string                  `json:"appUid,omitempty"`      // The Application's UID.
	EndpointId  string                   `json:"endpointId"`            // The Endpoint's ID.
	EndpointUid *string                  `json:"endpointUid,omitempty"` // The Endpoint's UID.
	FailSince   *time.Time               `json:"failSince,omitempty"`
	Trigger     *EndpointDisabledTrigger `json:"trigger,omitempty"`
}
