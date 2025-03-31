// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseApiTokenCensoredOut struct {
	Data         []ApiTokenCensoredOut `json:"data"`
	Done         bool                  `json:"done"`
	Iterator     *string               `json:"iterator,omitempty"`
	PrevIterator *string               `json:"prevIterator,omitempty"`
}
