// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApiTokenCensoredOut struct {
	CensoredToken string     `json:"censoredToken"`
	CreatedAt     time.Time  `json:"createdAt"`
	ExpiresAt     *time.Time `json:"expiresAt,omitempty"`
	Id            string     `json:"id"` // The ApplicationToken's ID.
	Name          *string    `json:"name,omitempty"`
	Scopes        []string   `json:"scopes,omitempty"`
}
