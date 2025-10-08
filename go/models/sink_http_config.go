// Package svix this file is @generated DO NOT EDIT
package models

type SinkHttpConfig struct {
	Headers *map[string]string `json:"headers,omitempty"`
	Key     *string            `json:"key,omitempty"`
	Url     string             `json:"url"`
}
