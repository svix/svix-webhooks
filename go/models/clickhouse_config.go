// Package svix this file is @generated DO NOT EDIT
package models

type ClickhouseConfig struct {
	Url       string  `json:"url"`                // The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
	Username  string  `json:"username"`           // Username to access Clickhouse
	Password  string  `json:"password"`           // Password to access Clickhouse
	Database  *string `json:"database,omitempty"` // The Clickhouse database to connect to
	TableName string  `json:"tableName"`          // The Clickhouse table to write to
}
