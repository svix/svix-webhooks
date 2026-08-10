// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/utils"
)

// When creating an StreamSinkPatch, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "amazonS3": Use AmazonS3ConfigPatch
//   - "azureBlobStorage": Use AzureBlobStorageConfigPatch
//   - "bigQuery": Use BigQueryConfigPatch
//   - "clickhouse": Use ClickhouseConfigPatch
//   - "eventBridge": Use EventBridgeConfigPatch
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigPatch
//   - "googleCloudStorage": Use GoogleCloudStorageConfigPatch
//   - "http": Use HttpConfigPatch
//   - "otelTracing": Use OtelTracingConfigPatch
//   - "rabbitMq": Use RabbitMqConfigPatch
//   - "redshift": Use RedshiftConfigPatch
//   - "snowflake": Use SnowflakeConfigPatch
//   - "sns": Use SnsConfigPatch
//   - "sqs": Use SqsConfigPatch
type StreamSinkPatch struct {
	Uid         utils.Nullable[string]       `json:"uid"` // The StreamSink's UID.
	Status      utils.Nullable[SinkStatusIn] `json:"status"`
	BatchSize   utils.Nullable[uint16]       `json:"batchSize"`
	MaxWaitSecs utils.Nullable[uint16]       `json:"maxWaitSecs"`
	EventTypes  []string                     `json:"eventTypes,omitempty"`
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
)

type StreamSinkPatchConfig interface {
	isStreamSinkPatchConfig()
}

func (emptyMap) isStreamSinkPatchConfig()                      {}
func (AzureBlobStorageConfigPatch) isStreamSinkPatchConfig()   {}
func (OtelTracingConfigPatch) isStreamSinkPatchConfig()        {}
func (HttpConfigPatch) isStreamSinkPatchConfig()               {}
func (AmazonS3ConfigPatch) isStreamSinkPatchConfig()           {}
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
	case "amazonS3":
		var c AmazonS3ConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
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
	case "http":
		var c HttpConfigPatch
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c OtelTracingConfigPatch
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
}
