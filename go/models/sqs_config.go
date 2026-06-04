// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for an SQS sink.
type SqsConfig struct {
	AccessKeyId     string  `json:"accessKeyId"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
	QueueUrl        string  `json:"queueUrl"`
	Region          string  `json:"region"`
	SecretAccessKey string  `json:"secretAccessKey"`
}
