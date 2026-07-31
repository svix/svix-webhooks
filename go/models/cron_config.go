// Package svix this file is @generated DO NOT EDIT
package models

type CronConfig struct {
	Schedule string `json:"schedule"`
	Payload  string `json:"payload"`
	// Override the default content-type.
	//
	// Recommended if the payload is not JSON.
	ContentType *string `json:"contentType,omitempty"`
}
