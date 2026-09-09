// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/utils"
)

// When creating an DestinationPatch, use the appropriate config structure based on the Type:
//   - "pollingEndpoint": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfigPatch
//   - "bigQuery": Use BigQueryConfigPatch
//   - "clickhouse": Use ClickhouseConfigPatch
//   - "eventBridge": Use EventBridgeConfigPatch
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigPatch
//   - "googleCloudStorage": Use GoogleCloudStorageConfigPatch
//   - "otelTracing": Use OtelTracingConfigPatch
//   - "postgres": Use PostgresConfigPatch
//   - "rabbitMq": Use RabbitMqConfigPatch
//   - "redshift": Use RedshiftConfigPatch
//   - "amazonS3": Use S3ConfigPatch
//   - "fifoEndpoint": Use SinkHttpConfigPatch
//   - "snowflake": Use SnowflakeConfigPatch
//   - "sns": Use SnsConfigPatch
//   - "sqs": Use SqsConfigPatch
type DestinationPatch struct {
	Uid         utils.Nullable[string]              `json:"uid"` // The Destination's UID.
	Status      utils.Nullable[DestinationStatusIn] `json:"status"`
	BatchSize   utils.Nullable[uint16]              `json:"batchSize"`
	MaxWaitSecs utils.Nullable[uint16]              `json:"maxWaitSecs"`
	EventTypes  []string                            `json:"eventTypes,omitempty"`
	Channels    []string                            `json:"channels,omitempty"`
	Metadata    *map[string]string                  `json:"metadata,omitempty"`
	Type        DestinationPatchType                `json:"type"`
	Config      DestinationPatchConfig              `json:"config"`
}

type DestinationPatchType string

const (
	DestinationPatchTypePollingEndpoint    DestinationPatchType = "pollingEndpoint"
	DestinationPatchTypeAzureBlobStorage   DestinationPatchType = "azureBlobStorage"
	DestinationPatchTypeOtelTracing        DestinationPatchType = "otelTracing"
	DestinationPatchTypeFifoEndpoint       DestinationPatchType = "fifoEndpoint"
	DestinationPatchTypeAmazonS3           DestinationPatchType = "amazonS3"
	DestinationPatchTypeGoogleCloudStorage DestinationPatchType = "googleCloudStorage"
	DestinationPatchTypeGoogleCloudPubSub  DestinationPatchType = "googleCloudPubSub"
	DestinationPatchTypeSqs                DestinationPatchType = "sqs"
	DestinationPatchTypeSns                DestinationPatchType = "sns"
	DestinationPatchTypeBigQuery           DestinationPatchType = "bigQuery"
	DestinationPatchTypeClickhouse         DestinationPatchType = "clickhouse"
	DestinationPatchTypeEventBridge        DestinationPatchType = "eventBridge"
	DestinationPatchTypeSnowflake          DestinationPatchType = "snowflake"
	DestinationPatchTypeRabbitMq           DestinationPatchType = "rabbitMq"
	DestinationPatchTypeRedshift           DestinationPatchType = "redshift"
	DestinationPatchTypePostgres           DestinationPatchType = "postgres"
)

type DestinationPatchConfig interface {
	isDestinationPatchConfig()
}

func (emptyMap) isDestinationPatchConfig()                      {}
func (AzureBlobStorageConfigPatch) isDestinationPatchConfig()   {}
func (OtelTracingConfigPatch) isDestinationPatchConfig()        {}
func (SinkHttpConfigPatch) isDestinationPatchConfig()           {}
func (S3ConfigPatch) isDestinationPatchConfig()                 {}
func (GoogleCloudStorageConfigPatch) isDestinationPatchConfig() {}
func (GoogleCloudPubSubConfigPatch) isDestinationPatchConfig()  {}
func (SqsConfigPatch) isDestinationPatchConfig()                {}
func (SnsConfigPatch) isDestinationPatchConfig()                {}
func (BigQueryConfigPatch) isDestinationPatchConfig()           {}
func (ClickhouseConfigPatch) isDestinationPatchConfig()         {}
func (EventBridgeConfigPatch) isDestinationPatchConfig()        {}
func (SnowflakeConfigPatch) isDestinationPatchConfig()          {}
func (RabbitMqConfigPatch) isDestinationPatchConfig()           {}
func (RedshiftConfigPatch) isDestinationPatchConfig()           {}
func (PostgresConfigPatch) isDestinationPatchConfig()           {}

func (i *DestinationPatch) UnmarshalJSON(data []byte) error {
	type Alias DestinationPatch
	aux := struct {
		*Alias
		Config json.RawMessage `json:"config"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Type {
	case "pollingEndpoint":
	case "azureBlobStorage":
		var c AzureBlobStorageConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "bigQuery":
		var c BigQueryConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "clickhouse":
		var c ClickhouseConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "eventBridge":
		var c EventBridgeConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudPubSub":
		var c GoogleCloudPubSubConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudStorage":
		var c GoogleCloudStorageConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c OtelTracingConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "postgres":
		var c PostgresConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rabbitMq":
		var c RabbitMqConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "redshift":
		var c RedshiftConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "amazonS3":
		var c S3ConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "fifoEndpoint":
		var c SinkHttpConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "snowflake":
		var c SnowflakeConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sns":
		var c SnsConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sqs":
		var c SqsConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

var DestinationPatchTypeWithNoConfig = map[string]bool{
	"pollingEndpoint": true,
}

func (i DestinationPatch) MarshalJSON() ([]byte, error) {
	type Alias DestinationPatch
	if _, found := DestinationPatchTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var DestinationPatchTypeFromString = map[string]DestinationPatchType{
	"pollingEndpoint":    DestinationPatchTypePollingEndpoint,
	"azureBlobStorage":   DestinationPatchTypeAzureBlobStorage,
	"otelTracing":        DestinationPatchTypeOtelTracing,
	"fifoEndpoint":       DestinationPatchTypeFifoEndpoint,
	"amazonS3":           DestinationPatchTypeAmazonS3,
	"googleCloudStorage": DestinationPatchTypeGoogleCloudStorage,
	"googleCloudPubSub":  DestinationPatchTypeGoogleCloudPubSub,
	"sqs":                DestinationPatchTypeSqs,
	"sns":                DestinationPatchTypeSns,
	"bigQuery":           DestinationPatchTypeBigQuery,
	"clickhouse":         DestinationPatchTypeClickhouse,
	"eventBridge":        DestinationPatchTypeEventBridge,
	"snowflake":          DestinationPatchTypeSnowflake,
	"rabbitMq":           DestinationPatchTypeRabbitMq,
	"redshift":           DestinationPatchTypeRedshift,
	"postgres":           DestinationPatchTypePostgres,
}
