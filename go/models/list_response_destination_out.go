// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseDestinationOut struct {
	Data         []DestinationOut `json:"data"`
	Iterator     *string          `json:"iterator,omitempty"`
	PrevIterator *string          `json:"prevIterator,omitempty"`
	Done         bool             `json:"done"`
}
