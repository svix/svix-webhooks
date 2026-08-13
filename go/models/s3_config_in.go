// Package svix this file is @generated DO NOT EDIT
package models

type S3ConfigIn struct {
	Bucket string `json:"bucket"`
	// Access key ID.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	// The region of the EventBridge bus.
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region      *string `json:"region,omitempty"`
	EndpointUrl *string `json:"endpointUrl,omitempty"`
}
