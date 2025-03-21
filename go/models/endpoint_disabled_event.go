// Package svix this file is @generated DO NOT EDIT
package models

// Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call.
type EndpointDisabledEvent struct {
	Data EndpointDisabledEventData `json:"data"`
	Type string                    `json:"type"`
}
