// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a Google Cloud BigQuery sink.
type BigQueryConfig struct {
	ProjectId   string `json:"projectId"`
	DatasetId   string `json:"datasetId"`
	TableId     string `json:"tableId"`
	Credentials string `json:"credentials"` // Google Cloud Credentials JSON Object as a string.
}
