// Package svix this file is @generated DO NOT EDIT
package models

type ClickhouseConfigIn struct {
	Url string `json:"url"` // The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
	// Username to access Clickhouse.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	Username *string `json:"username,omitempty"`
	// Password to access Clickhouse.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	Password  *string `json:"password,omitempty"`
	Database  *string `json:"database,omitempty"` // The Clickhouse database to connect to.
	TableName string  `json:"tableName"`          // The Clickhouse table to write to.
}
