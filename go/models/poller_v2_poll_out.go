// Package svix this file is @generated DO NOT EDIT
package models

type PollerV2PollOut struct {
	Data []PollerV2MessageOut `json:"data"`
	Done bool                 `json:"done"`
}
