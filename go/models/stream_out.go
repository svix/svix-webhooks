// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type StreamOut struct {
	Id        string            `json:"id"`             // The stream's ID.
	Uid       *string           `json:"uid,omitempty"`  // The stream's UID.
	Name      *string           `json:"name,omitempty"` // The stream's name.
	CreatedAt time.Time         `json:"createdAt"`
	UpdatedAt time.Time         `json:"updatedAt"`
	Metadata  map[string]string `json:"metadata"`
}
