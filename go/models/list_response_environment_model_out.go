// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseEnvironmentModelOut struct {
	Data         []EnvironmentModelOut `json:"data"`
	Done         bool                  `json:"done"`
	Iterator     *string               `json:"iterator,omitempty"`
	PrevIterator *string               `json:"prevIterator,omitempty"`
}
