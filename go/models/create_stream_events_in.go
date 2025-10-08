// Package svix this file is @generated DO NOT EDIT
package models

type CreateStreamEventsIn struct {
	Events []EventIn `json:"events"`
	// Optionally creates a new Stream alongside the events.
	//
	// If the stream id or uid that is used in the path already exists, this argument is ignored.
	Stream *StreamIn `json:"stream,omitempty"`
}
