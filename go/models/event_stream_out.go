// Package svix this file is @generated DO NOT EDIT
package models

type EventStreamOut struct {
	Data     []EventOut `json:"data"`
	Done     bool       `json:"done"`
	Iterator string     `json:"iterator"`
}
