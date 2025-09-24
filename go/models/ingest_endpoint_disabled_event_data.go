// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call.
type IngestEndpointDisabledEventData struct {
	EndpointId  string                   `json:"endpointId"`            // The Endpoint's ID.
	EndpointUid *string                  `json:"endpointUid,omitempty"` // The Endpoint's UID.
	FailSince   *time.Time               `json:"failSince,omitempty"`
	SourceId    string                   `json:"sourceId"` // The Source's ID.
	Trigger     *EndpointDisabledTrigger `json:"trigger,omitempty"`
}
