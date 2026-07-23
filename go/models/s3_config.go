// Package svix this file is @generated DO NOT EDIT
package models

type S3Config struct {
	Bucket          string  `json:"bucket"`
	AccessKeyId     string  `json:"accessKeyId"`
	SecretAccessKey string  `json:"secretAccessKey"`
	Region          string  `json:"region"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
}
