// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a SNS sink.
type SnsConfig struct {
	AccessKeyId     string  `json:"accessKeyId"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
	Region          string  `json:"region"`
	SecretAccessKey string  `json:"secretAccessKey"`
	TopicArn        string  `json:"topicArn"`
}
