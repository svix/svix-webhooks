// Package svix this file is @generated DO NOT EDIT
package models

type EndpointHeadersPatchIn struct {
	Headers       map[string]string `json:"headers"`
	DeleteHeaders []string          `json:"deleteHeaders,omitempty"` // A list of headers be be removed
}
