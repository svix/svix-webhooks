// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApiTokenCensoredOut struct {
	CensoredToken string     `json:"censoredToken"`
	Id            string     `json:"id"`
	Name          *string    `json:"name,omitempty"`
	CreatedAt     time.Time  `json:"createdAt"`
	ExpiresAt     *time.Time `json:"expiresAt,omitempty"`
	Scopes        []string   `json:"scopes,omitempty"`
}
