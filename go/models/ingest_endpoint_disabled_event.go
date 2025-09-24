// Package svix this file is @generated DO NOT EDIT
package models

// Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call.
type IngestEndpointDisabledEvent struct {
	Data IngestEndpointDisabledEventData `json:"data"`
	Type string                          `json:"type"`
}
