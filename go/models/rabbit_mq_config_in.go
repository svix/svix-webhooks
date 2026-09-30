// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a RabbitMq sink.
type RabbitMqConfigIn struct {
	// URI to connect to
	//
	// Note that the VHost must be percent-escaped, so a default URI would look
	// like `amqp://user:pass@host/%2F`
	Uri        string `json:"uri"`
	RoutingKey string `json:"routingKey"` // Routing key for message dispatch
	// If true, then dispatches will fail if there is no attached queue; if false, they are
	// silently dropped (this was previously the default)
	Mandatory *bool `json:"mandatory,omitempty"`
}
