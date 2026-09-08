// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type AutoConfigSubscriptionOut struct {
	CreatedAt     time.Time `json:"createdAt"`
	TokenCensored string    `json:"tokenCensored"`
	Id            string    `json:"id"`               // The AutoConfigSubscription's ID.
	EndpId        *string   `json:"endpId,omitempty"` // The Endpoint's ID.
	DestId        *string   `json:"destId,omitempty"` // The StreamSink's ID.
	Status        Status    `json:"status"`
}
