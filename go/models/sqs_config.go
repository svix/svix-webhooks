// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for an SQS sink.
type SqsConfig struct {
	QueueUrl        string  `json:"queueUrl"`
	Region          string  `json:"region"`
	AccessKeyId     string  `json:"accessKeyId"`
	SecretAccessKey string  `json:"secretAccessKey"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
}
