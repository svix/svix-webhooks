// Package svix this file is @generated DO NOT EDIT
package models

type CronConfig struct {
	// Override the default content-type.
	//
	// Recommended if the payload is not JSON.
	ContentType *string `json:"contentType,omitempty"`
	Payload     string  `json:"payload"`
	Schedule    string  `json:"schedule"`
}
