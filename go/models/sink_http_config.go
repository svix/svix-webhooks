// Package svix this file is @generated DO NOT EDIT
package models

type SinkHttpConfig struct {
	Url     string             `json:"url"`
	Headers *map[string]string `json:"headers,omitempty"`
	Key     *string            `json:"key,omitempty"`
}
