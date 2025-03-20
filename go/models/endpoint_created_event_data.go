// Package svix this file is @generated DO NOT EDIT
package models

// Sent when an endpoint is created, updated, or deleted
type EndpointCreatedEventData struct {
	AppId       string  `json:"appId"`                 // The Application's ID.
	AppUid      *string `json:"appUid,omitempty"`      // The Application's UID.
	EndpointId  string  `json:"endpointId"`            // The Endpoint's ID.
	EndpointUid *string `json:"endpointUid,omitempty"` // The Endpoint's UID.
}
