// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"time"
)

// When creating an StreamSinkOut, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfig
//   - "bigQuery": Use BigQueryConfig
//   - "clickhouse": Use ClickhouseConfig
//   - "eventBridge": Use EventBridgeConfig
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfig
//   - "googleCloudStorage": Use GoogleCloudStorageConfig
//   - "rabbitMq": Use RabbitMqConfig
//   - "redshift": Use RedshiftConfig
//   - "amazonS3": Use S3Config
//   - "http": Use SinkHttpConfig
//   - "otelTracing": Use SinkOtelV1Config
//   - "snowflake": Use SnowflakeConfig
//   - "sns": Use SnsConfig
//   - "sqs": Use SqsConfig
type StreamSinkOut struct {
	BatchSize       int32               `json:"batchSize"`
	CreatedAt       time.Time           `json:"createdAt"`
	CurrentIterator string              `json:"currentIterator"`
	EventTypes      []string            `json:"eventTypes,omitempty"`
	FailureReason   *string             `json:"failureReason,omitempty"`
	Id              string              `json:"id"` // The sink's ID.
	MaxWaitSecs     int32               `json:"maxWaitSecs"`
	Metadata        map[string]string   `json:"metadata"`
	NextRetryAt     *time.Time          `json:"nextRetryAt,omitempty"`
	Status          SinkStatus          `json:"status"`
	Uid             *string             `json:"uid,omitempty"` // The sink's UID.
	UpdatedAt       time.Time           `json:"updatedAt"`
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
	StreamSinkOutTypeGoogleCloudStorage StreamSinkOutType = "googleCloudStorage"
	StreamSinkOutTypeGoogleCloudPubSub  StreamSinkOutType = "googleCloudPubSub"
	StreamSinkOutTypeSqs                StreamSinkOutType = "sqs"
	StreamSinkOutTypeSns                StreamSinkOutType = "sns"
	StreamSinkOutTypeBigQuery           StreamSinkOutType = "bigQuery"
	StreamSinkOutTypeClickhouse         StreamSinkOutType = "clickhouse"
	StreamSinkOutTypeEventBridge        StreamSinkOutType = "eventBridge"
	StreamSinkOutTypeSnowflake          StreamSinkOutType = "snowflake"
	StreamSinkOutTypeRabbitMq           StreamSinkOutType = "rabbitMq"
	StreamSinkOutTypeRedshift           StreamSinkOutType = "redshift"
)

type StreamSinkOutConfig interface {
	isStreamSinkOutConfig()
}

func (emptyMap) isStreamSinkOutConfig()                 {}
func (AzureBlobStorageConfig) isStreamSinkOutConfig()   {}
func (SinkOtelV1Config) isStreamSinkOutConfig()         {}
func (SinkHttpConfig) isStreamSinkOutConfig()           {}
func (S3Config) isStreamSinkOutConfig()                 {}
func (GoogleCloudStorageConfig) isStreamSinkOutConfig() {}
func (GoogleCloudPubSubConfig) isStreamSinkOutConfig()  {}
func (SqsConfig) isStreamSinkOutConfig()                {}
func (SnsConfig) isStreamSinkOutConfig()                {}
func (BigQueryConfig) isStreamSinkOutConfig()           {}
func (ClickhouseConfig) isStreamSinkOutConfig()         {}
func (EventBridgeConfig) isStreamSinkOutConfig()        {}
func (SnowflakeConfig) isStreamSinkOutConfig()          {}
func (RabbitMqConfig) isStreamSinkOutConfig()           {}
func (RedshiftConfig) isStreamSinkOutConfig()           {}

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
		var c AzureBlobStorageConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "bigQuery":
		var c BigQueryConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "clickhouse":
		var c ClickhouseConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "eventBridge":
		var c EventBridgeConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudPubSub":
		var c GoogleCloudPubSubConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudStorage":
		var c GoogleCloudStorageConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rabbitMq":
		var c RabbitMqConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "redshift":
		var c RedshiftConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "amazonS3":
		var c S3Config
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "http":
		var c SinkHttpConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c SinkOtelV1Config
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "snowflake":
		var c SnowflakeConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sns":
		var c SnsConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sqs":
		var c SqsConfig
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
	"googleCloudStorage": StreamSinkOutTypeGoogleCloudStorage,
	"googleCloudPubSub":  StreamSinkOutTypeGoogleCloudPubSub,
	"sqs":                StreamSinkOutTypeSqs,
	"sns":                StreamSinkOutTypeSns,
	"bigQuery":           StreamSinkOutTypeBigQuery,
	"clickhouse":         StreamSinkOutTypeClickhouse,
	"eventBridge":        StreamSinkOutTypeEventBridge,
	"snowflake":          StreamSinkOutTypeSnowflake,
	"rabbitMq":           StreamSinkOutTypeRabbitMq,
	"redshift":           StreamSinkOutTypeRedshift,
}
