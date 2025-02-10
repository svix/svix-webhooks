// Package svix this file is @generated DO NOT EDIT
package models

type ApplicationIn struct {
	Metadata  *map[string]string `json:"metadata,omitempty"`
	Name      string             `json:"name"`
	RateLimit *uint16            `json:"rateLimit,omitempty"`
	// Optional unique identifier for the application.
	Uid *string `json:"uid,omitempty"`
}
