// Package svix this file is @generated DO NOT EDIT
package models

type S3ConfigOut struct {
	Bucket      string  `json:"bucket"`
	AccessKeyId string  `json:"accessKeyId"`
	Region      string  `json:"region"`
	EndpointUrl *string `json:"endpointUrl,omitempty"`
}
