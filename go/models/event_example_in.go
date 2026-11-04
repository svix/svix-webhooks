// Package svix this file is @generated DO NOT EDIT
package models

type EventExampleIn struct {
	EventType string `json:"eventType"` // The event type's name
	// If the event type schema contains an array of examples, chooses which one to send.
	//
	// Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
	ExampleIndex *uint64 `json:"exampleIndex,omitempty"`
	// Custom payload to send as an example
	//
	// This is only available to allow-listed customers and should otherwise not be passed. Please
	// contact us if you need access to this functionality
	Payload *map[string]any `json:"payload,omitempty"`
}
