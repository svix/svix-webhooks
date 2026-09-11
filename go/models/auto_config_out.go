// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type AutoConfigOut struct {
	CreatedAt time.Time `json:"createdAt"`
	Token     string    `json:"token"`
	Id        string    `json:"id"` // The AutoConfigSubscription's ID.
}
