// Package svix this file is @generated DO NOT EDIT
package models

type AzureBlobStorageConfigIn struct {
	Container string `json:"container"`
	Account   string `json:"account"`
	// Access key.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	AccessKey *string `json:"accessKey,omitempty"`
}
