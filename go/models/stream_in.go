// Package svix this file is @generated DO NOT EDIT
package models

type StreamIn struct {
	Metadata *map[string]string `json:"metadata,omitempty"`
	Name     string             `json:"name"`          // The stream's name.
	Uid      *string            `json:"uid,omitempty"` // An optional unique identifier for the stream.
}
