// Package svix this file is @generated DO NOT EDIT
package models

type SubscribeIn struct {
	Endpoint *EndpointIn         `json:"endpoint,omitempty"`
	Sink     *AutoConfigSinkType `json:"sink,omitempty"`
}
