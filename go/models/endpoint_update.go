// Package svix this file is @generated DO NOT EDIT
package models

type EndpointUpdate struct {
	// List of message channels this endpoint listens to (omit for all).
	Channels    []string           `json:"channels,omitempty"`
	Description *string            `json:"description,omitempty"`
	Disabled    *bool              `json:"disabled,omitempty"`
	FilterTypes []string           `json:"filterTypes,omitempty"`
	Metadata    *map[string]string `json:"metadata,omitempty"`
	RateLimit   *uint16            `json:"rateLimit,omitempty"`
	// Optional unique identifier for the endpoint.
	Uid     *string `json:"uid,omitempty"`
	Url     string  `json:"url"`
	Version *uint16 `json:"version,omitempty"`
}
