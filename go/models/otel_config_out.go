// Package svix this file is @generated DO NOT EDIT
package models

type OtelConfigOut struct {
	Url               *string            `json:"url,omitempty"`
	AdditionalHeaders *map[string]string `json:"additionalHeaders,omitempty"`
	SvixManaged       bool               `json:"svixManaged"`
}
