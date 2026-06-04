// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a Google Cloud BigQuery sink.
type BigQueryConfig struct {
	Credentials string `json:"credentials"` // Google Cloud Credentials JSON Object as a string.
	DatasetId   string `json:"datasetId"`
	ProjectId   string `json:"projectId"`
	TableId     string `json:"tableId"`
}
