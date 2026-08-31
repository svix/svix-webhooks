// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type AdvancedEndpointType string

const (
	ADVANCEDENDPOINTTYPE_POLLER               AdvancedEndpointType = "poller"
	ADVANCEDENDPOINTTYPE_HTTP                 AdvancedEndpointType = "http"
	ADVANCEDENDPOINTTYPE_AMAZON_S3            AdvancedEndpointType = "amazonS3"
	ADVANCEDENDPOINTTYPE_AZURE_BLOB_STORAGE   AdvancedEndpointType = "azureBlobStorage"
	ADVANCEDENDPOINTTYPE_GOOGLE_CLOUD_STORAGE AdvancedEndpointType = "googleCloudStorage"
	ADVANCEDENDPOINTTYPE_GOOGLE_CLOUD_PUB_SUB AdvancedEndpointType = "googleCloudPubSub"
	ADVANCEDENDPOINTTYPE_SQS                  AdvancedEndpointType = "sqs"
	ADVANCEDENDPOINTTYPE_SNS                  AdvancedEndpointType = "sns"
	ADVANCEDENDPOINTTYPE_BIG_QUERY            AdvancedEndpointType = "bigQuery"
	ADVANCEDENDPOINTTYPE_CLICKHOUSE           AdvancedEndpointType = "clickhouse"
	ADVANCEDENDPOINTTYPE_EVENT_BRIDGE         AdvancedEndpointType = "eventBridge"
	ADVANCEDENDPOINTTYPE_SNOWFLAKE            AdvancedEndpointType = "snowflake"
	ADVANCEDENDPOINTTYPE_RABBIT_MQ            AdvancedEndpointType = "rabbitMq"
	ADVANCEDENDPOINTTYPE_REDSHIFT             AdvancedEndpointType = "redshift"
	ADVANCEDENDPOINTTYPE_OTEL_TRACING         AdvancedEndpointType = "otelTracing"
	ADVANCEDENDPOINTTYPE_POSTGRES             AdvancedEndpointType = "postgres"
)

var allowedAdvancedEndpointType = []AdvancedEndpointType{
	"poller",
	"http",
	"amazonS3",
	"azureBlobStorage",
	"googleCloudStorage",
	"googleCloudPubSub",
	"sqs",
	"sns",
	"bigQuery",
	"clickhouse",
	"eventBridge",
	"snowflake",
	"rabbitMq",
	"redshift",
	"otelTracing",
	"postgres",
}

func (v *AdvancedEndpointType) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := AdvancedEndpointType(value)
	if slices.Contains(allowedAdvancedEndpointType, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid AdvancedEndpointType", value)

}

var AdvancedEndpointTypeFromString = map[string]AdvancedEndpointType{
	"poller":             ADVANCEDENDPOINTTYPE_POLLER,
	"http":               ADVANCEDENDPOINTTYPE_HTTP,
	"amazonS3":           ADVANCEDENDPOINTTYPE_AMAZON_S3,
	"azureBlobStorage":   ADVANCEDENDPOINTTYPE_AZURE_BLOB_STORAGE,
	"googleCloudStorage": ADVANCEDENDPOINTTYPE_GOOGLE_CLOUD_STORAGE,
	"googleCloudPubSub":  ADVANCEDENDPOINTTYPE_GOOGLE_CLOUD_PUB_SUB,
	"sqs":                ADVANCEDENDPOINTTYPE_SQS,
	"sns":                ADVANCEDENDPOINTTYPE_SNS,
	"bigQuery":           ADVANCEDENDPOINTTYPE_BIG_QUERY,
	"clickhouse":         ADVANCEDENDPOINTTYPE_CLICKHOUSE,
	"eventBridge":        ADVANCEDENDPOINTTYPE_EVENT_BRIDGE,
	"snowflake":          ADVANCEDENDPOINTTYPE_SNOWFLAKE,
	"rabbitMq":           ADVANCEDENDPOINTTYPE_RABBIT_MQ,
	"redshift":           ADVANCEDENDPOINTTYPE_REDSHIFT,
	"otelTracing":        ADVANCEDENDPOINTTYPE_OTEL_TRACING,
	"postgres":           ADVANCEDENDPOINTTYPE_POSTGRES,
}
