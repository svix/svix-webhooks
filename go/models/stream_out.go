// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type StreamOut struct {
	CreatedAt time.Time         `json:"createdAt"`
	Id        string            `json:"id"` // The stream's ID.
	Metadata  map[string]string `json:"metadata"`
	Name      *string           `json:"name,omitempty"` // The stream's name.
	Uid       *string           `json:"uid,omitempty"`  // The stream's UID.
	UpdatedAt time.Time         `json:"updatedAt"`
}
