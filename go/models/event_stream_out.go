// Package svix this file is @generated DO NOT EDIT
package models

type EventStreamOut struct {
	Data     []EventOut `json:"data"`
	Iterator string     `json:"iterator"`
	Done     bool       `json:"done"`
}
