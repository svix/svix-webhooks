// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a Snowflake sink.
type SnowflakeConfig struct {
	AccountIdentifier string `json:"accountIdentifier"` // Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen.
	// Database name.
	//
	// Only required if not using transformations.
	DbName *string `json:"dbName,omitempty"`
	// PEM-encoded private key used for signing token-based requests to the Snowflake API.
	//
	// Beginning/end delimiters are not required.
	PrivateKey string `json:"privateKey"`
	// Schema name.
	//
	// Only required if not using transformations.
	SchemaName *string `json:"schemaName,omitempty"`
	// Table name.
	//
	// Only required if not using transformations.
	TableName *string `json:"tableName,omitempty"`
	UserId    string  `json:"userId"` // The Snowflake user id.
}
