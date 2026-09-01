// Package svix this file is @generated DO NOT EDIT
package models

type S3ConfigIn struct {
	Bucket string `json:"bucket"`
	// Access key ID.
	//
	// Required (along with `secret_access_key`) if `role_arn` is null
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Required (along with `access_key_id`) if `role_arn` is null
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	// The region of the S3 bucket
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region      *string `json:"region,omitempty"`
	EndpointUrl *string `json:"endpointUrl,omitempty"`
	RoleArn     *string `json:"roleArn,omitempty"` // Role ARN for delegated authentication
	// Shared secret passed as the STS ExternalId.
	//
	// Required if `role_arn` is not null.
	ExternalId *string `json:"externalId,omitempty"`
}
