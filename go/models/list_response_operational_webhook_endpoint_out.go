// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseOperationalWebhookEndpointOut struct {
	Data         []OperationalWebhookEndpointOut `json:"data"`
	Iterator     *string                         `json:"iterator,omitempty"`
	PrevIterator *string                         `json:"prevIterator,omitempty"`
	Done         bool                            `json:"done"`
}
