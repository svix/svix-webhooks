// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseApplicationOut struct {
	Data         []ApplicationOut `json:"data"`
	Iterator     *string          `json:"iterator,omitempty"`
	PrevIterator *string          `json:"prevIterator,omitempty"`
	Done         bool             `json:"done"`
}
