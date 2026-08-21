// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"time"
)

// When creating an StreamSinkOut, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfigOut
//   - "bigQuery": Use BigQueryConfigOut
//   - "clickhouse": Use ClickhouseConfigOut
//   - "eventBridge": Use EventBridgeConfigOut
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigOut
//   - "googleCloudStorage": Use GoogleCloudStorageConfigOut
//   - "otelTracing": Use OtelTracingConfigOut
//   - "rabbitMq": Use RabbitMqConfigOut
//   - "redshift": Use RedshiftConfigOut
//   - "amazonS3": Use S3ConfigOut
//   - "http": Use SinkHttpConfigOut
//   - "snowflake": Use SnowflakeConfigOut
//   - "sns": Use SnsConfigOut
//   - "sqs": Use SqsConfigOut
type StreamSinkOut struct {
	Id              string              `json:"id"`            // The sink's ID.
	Uid             *string             `json:"uid,omitempty"` // The sink's UID.
	Status          SinkStatus          `json:"status"`
	CurrentIterator string              `json:"currentIterator"`
	FailureReason   *string             `json:"failureReason,omitempty"`
	CreatedAt       time.Time           `json:"createdAt"`
	UpdatedAt       time.Time           `json:"updatedAt"`
	BatchSize       int32               `json:"batchSize"`
	MaxWaitSecs     int32               `json:"maxWaitSecs"`
	EventTypes      []string            `json:"eventTypes,omitempty"`
	Channels        []string            `json:"channels,omitempty"`
	NextRetryAt     *time.Time          `json:"nextRetryAt,omitempty"`
	Metadata        map[string]string   `json:"metadata"`
	Type            StreamSinkOutType   `json:"type"`
	Config          StreamSinkOutConfig `json:"config"`
}

type StreamSinkOutType string

const (
	StreamSinkOutTypePoller             StreamSinkOutType = "poller"
	StreamSinkOutTypeAzureBlobStorage   StreamSinkOutType = "azureBlobStorage"
	StreamSinkOutTypeOtelTracing        StreamSinkOutType = "otelTracing"
	StreamSinkOutTypeHttp               StreamSinkOutType = "http"
	StreamSinkOutTypeAmazonS3           StreamSinkOutType = "amazonS3"
	StreamSinkOutTypeSnowflake          StreamSinkOutType = "snowflake"
	StreamSinkOutTypeGoogleCloudStorage StreamSinkOutType = "googleCloudStorage"
	StreamSinkOutTypeGoogleCloudPubSub  StreamSinkOutType = "googleCloudPubSub"
	StreamSinkOutTypeRedshift           StreamSinkOutType = "redshift"
	StreamSinkOutTypeBigQuery           StreamSinkOutType = "bigQuery"
	StreamSinkOutTypeClickhouse         StreamSinkOutType = "clickhouse"
	StreamSinkOutTypeRabbitMq           StreamSinkOutType = "rabbitMq"
	StreamSinkOutTypeSqs                StreamSinkOutType = "sqs"
	StreamSinkOutTypeEventBridge        StreamSinkOutType = "eventBridge"
	StreamSinkOutTypeSns                StreamSinkOutType = "sns"
)

type StreamSinkOutConfig interface {
	isStreamSinkOutConfig()
}

func (emptyMap) isStreamSinkOutConfig()                    {}
func (AzureBlobStorageConfigOut) isStreamSinkOutConfig()   {}
func (OtelTracingConfigOut) isStreamSinkOutConfig()        {}
func (SinkHttpConfigOut) isStreamSinkOutConfig()           {}
func (S3ConfigOut) isStreamSinkOutConfig()                 {}
func (SnowflakeConfigOut) isStreamSinkOutConfig()          {}
func (GoogleCloudStorageConfigOut) isStreamSinkOutConfig() {}
func (GoogleCloudPubSubConfigOut) isStreamSinkOutConfig()  {}
func (RedshiftConfigOut) isStreamSinkOutConfig()           {}
func (BigQueryConfigOut) isStreamSinkOutConfig()           {}
func (ClickhouseConfigOut) isStreamSinkOutConfig()         {}
func (RabbitMqConfigOut) isStreamSinkOutConfig()           {}
func (SqsConfigOut) isStreamSinkOutConfig()                {}
func (EventBridgeConfigOut) isStreamSinkOutConfig()        {}
func (SnsConfigOut) isStreamSinkOutConfig()                {}

func (i *StreamSinkOut) UnmarshalJSON(data []byte) error {
	type Alias StreamSinkOut
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
		var c AzureBlobStorageConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "bigQuery":
		var c BigQueryConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "clickhouse":
		var c ClickhouseConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "eventBridge":
		var c EventBridgeConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudPubSub":
		var c GoogleCloudPubSubConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudStorage":
		var c GoogleCloudStorageConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c OtelTracingConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rabbitMq":
		var c RabbitMqConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "redshift":
		var c RedshiftConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "amazonS3":
		var c S3ConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "http":
		var c SinkHttpConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "snowflake":
		var c SnowflakeConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sns":
		var c SnsConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sqs":
		var c SqsConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

var StreamSinkOutTypeWithNoConfig = map[string]bool{
	"poller": true,
}

func (i StreamSinkOut) MarshalJSON() ([]byte, error) {
	type Alias StreamSinkOut
	if _, found := StreamSinkOutTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var StreamSinkOutTypeFromString = map[string]StreamSinkOutType{
	"poller":             StreamSinkOutTypePoller,
	"azureBlobStorage":   StreamSinkOutTypeAzureBlobStorage,
	"otelTracing":        StreamSinkOutTypeOtelTracing,
	"http":               StreamSinkOutTypeHttp,
	"amazonS3":           StreamSinkOutTypeAmazonS3,
	"snowflake":          StreamSinkOutTypeSnowflake,
	"googleCloudStorage": StreamSinkOutTypeGoogleCloudStorage,
	"googleCloudPubSub":  StreamSinkOutTypeGoogleCloudPubSub,
	"redshift":           StreamSinkOutTypeRedshift,
	"bigQuery":           StreamSinkOutTypeBigQuery,
	"clickhouse":         StreamSinkOutTypeClickhouse,
	"rabbitMq":           StreamSinkOutTypeRabbitMq,
	"sqs":                StreamSinkOutTypeSqs,
	"eventBridge":        StreamSinkOutTypeEventBridge,
	"sns":                StreamSinkOutTypeSns,
}
