// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a Google Cloud Storage sink.
//
// Write stream events into the named bucket using the supplied Google Cloud credentials.
type GoogleCloudStorageConfig struct {
	Bucket      string `json:"bucket"`
	Credentials string `json:"credentials"` // Google Cloud Credentials JSON Object as a string.
}
