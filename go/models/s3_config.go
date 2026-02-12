// Package svix this file is @generated DO NOT EDIT
package models

type S3Config struct {
	AccessKeyId     string  `json:"accessKeyId"`
	Bucket          string  `json:"bucket"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
	Region          string  `json:"region"`
	SecretAccessKey string  `json:"secretAccessKey"`
}
