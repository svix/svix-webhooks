// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/utils"
)

// When creating an StreamSinkPatch, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
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
//   - "http": Use SinkHttpConfigPatch
//   - "snowflake": Use SnowflakeConfigPatch
//   - "sns": Use SnsConfigPatch
//   - "sqs": Use SqsConfigPatch
type StreamSinkPatch struct {
	Uid         utils.Nullable[string]       `json:"uid"` // The StreamSink's UID.
	Status      utils.Nullable[SinkStatusIn] `json:"status"`
	BatchSize   utils.Nullable[uint16]       `json:"batchSize"`
	MaxWaitSecs utils.Nullable[uint16]       `json:"maxWaitSecs"`
	EventTypes  []string                     `json:"eventTypes,omitempty"`
	Channels    []string                     `json:"channels,omitempty"`
	Metadata    *map[string]string           `json:"metadata,omitempty"`
	Type        StreamSinkPatchType          `json:"type"`
	Config      StreamSinkPatchConfig        `json:"config"`
}

type StreamSinkPatchType string

const (
	StreamSinkPatchTypePoller             StreamSinkPatchType = "poller"
	StreamSinkPatchTypeAzureBlobStorage   StreamSinkPatchType = "azureBlobStorage"
	StreamSinkPatchTypeOtelTracing        StreamSinkPatchType = "otelTracing"
	StreamSinkPatchTypeHttp               StreamSinkPatchType = "http"
	StreamSinkPatchTypeAmazonS3           StreamSinkPatchType = "amazonS3"
	StreamSinkPatchTypeGoogleCloudStorage StreamSinkPatchType = "googleCloudStorage"
	StreamSinkPatchTypeGoogleCloudPubSub  StreamSinkPatchType = "googleCloudPubSub"
	StreamSinkPatchTypeSqs                StreamSinkPatchType = "sqs"
	StreamSinkPatchTypeSns                StreamSinkPatchType = "sns"
	StreamSinkPatchTypeBigQuery           StreamSinkPatchType = "bigQuery"
	StreamSinkPatchTypeClickhouse         StreamSinkPatchType = "clickhouse"
	StreamSinkPatchTypeEventBridge        StreamSinkPatchType = "eventBridge"
	StreamSinkPatchTypeSnowflake          StreamSinkPatchType = "snowflake"
	StreamSinkPatchTypeRabbitMq           StreamSinkPatchType = "rabbitMq"
	StreamSinkPatchTypeRedshift           StreamSinkPatchType = "redshift"
	StreamSinkPatchTypePostgres           StreamSinkPatchType = "postgres"
)

type StreamSinkPatchConfig interface {
	isStreamSinkPatchConfig()
}

func (emptyMap) isStreamSinkPatchConfig()                      {}
func (AzureBlobStorageConfigPatch) isStreamSinkPatchConfig()   {}
func (OtelTracingConfigPatch) isStreamSinkPatchConfig()        {}
func (SinkHttpConfigPatch) isStreamSinkPatchConfig()           {}
func (S3ConfigPatch) isStreamSinkPatchConfig()                 {}
func (GoogleCloudStorageConfigPatch) isStreamSinkPatchConfig() {}
func (GoogleCloudPubSubConfigPatch) isStreamSinkPatchConfig()  {}
func (SqsConfigPatch) isStreamSinkPatchConfig()                {}
func (SnsConfigPatch) isStreamSinkPatchConfig()                {}
func (BigQueryConfigPatch) isStreamSinkPatchConfig()           {}
func (ClickhouseConfigPatch) isStreamSinkPatchConfig()         {}
func (EventBridgeConfigPatch) isStreamSinkPatchConfig()        {}
func (SnowflakeConfigPatch) isStreamSinkPatchConfig()          {}
func (RabbitMqConfigPatch) isStreamSinkPatchConfig()           {}
func (RedshiftConfigPatch) isStreamSinkPatchConfig()           {}
func (PostgresConfigPatch) isStreamSinkPatchConfig()           {}

func (i *StreamSinkPatch) UnmarshalJSON(data []byte) error {
	type Alias StreamSinkPatch
	aux := struct {
		*Alias
		Config json.RawMessage `json:"config"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Type {
	case "poller":
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
	case "http":
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

var StreamSinkPatchTypeWithNoConfig = map[string]bool{
	"poller": true,
}

func (i StreamSinkPatch) MarshalJSON() ([]byte, error) {
	type Alias StreamSinkPatch
	if _, found := StreamSinkPatchTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var StreamSinkPatchTypeFromString = map[string]StreamSinkPatchType{
	"poller":             StreamSinkPatchTypePoller,
	"azureBlobStorage":   StreamSinkPatchTypeAzureBlobStorage,
	"otelTracing":        StreamSinkPatchTypeOtelTracing,
	"http":               StreamSinkPatchTypeHttp,
	"amazonS3":           StreamSinkPatchTypeAmazonS3,
	"googleCloudStorage": StreamSinkPatchTypeGoogleCloudStorage,
	"googleCloudPubSub":  StreamSinkPatchTypeGoogleCloudPubSub,
	"sqs":                StreamSinkPatchTypeSqs,
	"sns":                StreamSinkPatchTypeSns,
	"bigQuery":           StreamSinkPatchTypeBigQuery,
	"clickhouse":         StreamSinkPatchTypeClickhouse,
	"eventBridge":        StreamSinkPatchTypeEventBridge,
	"snowflake":          StreamSinkPatchTypeSnowflake,
	"rabbitMq":           StreamSinkPatchTypeRabbitMq,
	"redshift":           StreamSinkPatchTypeRedshift,
	"postgres":           StreamSinkPatchTypePostgres,
}
