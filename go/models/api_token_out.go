// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApiTokenOut struct {
	CreatedAt time.Time  `json:"createdAt"`
	ExpiresAt *time.Time `json:"expiresAt,omitempty"`
	Id        string     `json:"id"` // The GlobalApplicationToken's ID.
	Name      *string    `json:"name,omitempty"`
	Scopes    []string   `json:"scopes,omitempty"`
	Token     string     `json:"token"`
}
