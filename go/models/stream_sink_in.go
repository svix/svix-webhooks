// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
)

// When creating an StreamSinkIn, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfigIn
//   - "bigQuery": Use BigQueryConfigIn
//   - "clickhouse": Use ClickhouseConfigIn
//   - "eventBridge": Use EventBridgeConfigIn
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigIn
//   - "googleCloudStorage": Use GoogleCloudStorageConfigIn
//   - "rabbitMq": Use RabbitMqConfigIn
//   - "redshift": Use RedshiftConfigIn
//   - "amazonS3": Use S3ConfigIn
//   - "http": Use SinkHttpConfigIn
//   - "otelTracing": Use SinkOtelTracingConfigIn
//   - "snowflake": Use SnowflakeConfigIn
//   - "sns": Use SnsConfigIn
//   - "sqs": Use SqsConfigIn
type StreamSinkIn struct {
	Uid *string `json:"uid,omitempty"` // An optional unique identifier for the sink.
	// Whether the sink will receive events.
	//
	// If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.
	//
	// If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled.
	Status    *SinkStatusIn `json:"status,omitempty"`
	BatchSize *uint16       `json:"batchSize,omitempty"` // How many events will be batched in a request to the Sink.
	// How long to wait before a batch of events is sent, if the `batchSize` is not reached.
	//
	// For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.
	//
	// Note that we will never send an empty batch of events to the Sink.
	MaxWaitSecs *uint16            `json:"maxWaitSecs,omitempty"`
	EventTypes  []string           `json:"eventTypes,omitempty"` // A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events.
	Metadata    *map[string]string `json:"metadata,omitempty"`
	Type        StreamSinkInType   `json:"type"`
	Config      StreamSinkInConfig `json:"config"`
}

type StreamSinkInType string

const (
	StreamSinkInTypePoller             StreamSinkInType = "poller"
	StreamSinkInTypeAzureBlobStorage   StreamSinkInType = "azureBlobStorage"
	StreamSinkInTypeOtelTracing        StreamSinkInType = "otelTracing"
	StreamSinkInTypeHttp               StreamSinkInType = "http"
	StreamSinkInTypeAmazonS3           StreamSinkInType = "amazonS3"
	StreamSinkInTypeGoogleCloudStorage StreamSinkInType = "googleCloudStorage"
	StreamSinkInTypeGoogleCloudPubSub  StreamSinkInType = "googleCloudPubSub"
	StreamSinkInTypeSqs                StreamSinkInType = "sqs"
	StreamSinkInTypeSns                StreamSinkInType = "sns"
	StreamSinkInTypeBigQuery           StreamSinkInType = "bigQuery"
	StreamSinkInTypeClickhouse         StreamSinkInType = "clickhouse"
	StreamSinkInTypeEventBridge        StreamSinkInType = "eventBridge"
	StreamSinkInTypeSnowflake          StreamSinkInType = "snowflake"
	StreamSinkInTypeRabbitMq           StreamSinkInType = "rabbitMq"
	StreamSinkInTypeRedshift           StreamSinkInType = "redshift"
)

type StreamSinkInConfig interface {
	isStreamSinkInConfig()
}

func (emptyMap) isStreamSinkInConfig()                   {}
func (AzureBlobStorageConfigIn) isStreamSinkInConfig()   {}
func (SinkOtelTracingConfigIn) isStreamSinkInConfig()    {}
func (SinkHttpConfigIn) isStreamSinkInConfig()           {}
func (S3ConfigIn) isStreamSinkInConfig()                 {}
func (GoogleCloudStorageConfigIn) isStreamSinkInConfig() {}
func (GoogleCloudPubSubConfigIn) isStreamSinkInConfig()  {}
func (SqsConfigIn) isStreamSinkInConfig()                {}
func (SnsConfigIn) isStreamSinkInConfig()                {}
func (BigQueryConfigIn) isStreamSinkInConfig()           {}
func (ClickhouseConfigIn) isStreamSinkInConfig()         {}
func (EventBridgeConfigIn) isStreamSinkInConfig()        {}
func (SnowflakeConfigIn) isStreamSinkInConfig()          {}
func (RabbitMqConfigIn) isStreamSinkInConfig()           {}
func (RedshiftConfigIn) isStreamSinkInConfig()           {}

func (i *StreamSinkIn) UnmarshalJSON(data []byte) error {
	type Alias StreamSinkIn
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
		var c AzureBlobStorageConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "bigQuery":
		var c BigQueryConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "clickhouse":
		var c ClickhouseConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "eventBridge":
		var c EventBridgeConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudPubSub":
		var c GoogleCloudPubSubConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudStorage":
		var c GoogleCloudStorageConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rabbitMq":
		var c RabbitMqConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "redshift":
		var c RedshiftConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "amazonS3":
		var c S3ConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "http":
		var c SinkHttpConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c SinkOtelTracingConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "snowflake":
		var c SnowflakeConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sns":
		var c SnsConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "sqs":
		var c SqsConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

var StreamSinkInTypeWithNoConfig = map[string]bool{
	"poller": true,
}

func (i StreamSinkIn) MarshalJSON() ([]byte, error) {
	type Alias StreamSinkIn
	if _, found := StreamSinkInTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var StreamSinkInTypeFromString = map[string]StreamSinkInType{
	"poller":             StreamSinkInTypePoller,
	"azureBlobStorage":   StreamSinkInTypeAzureBlobStorage,
	"otelTracing":        StreamSinkInTypeOtelTracing,
	"http":               StreamSinkInTypeHttp,
	"amazonS3":           StreamSinkInTypeAmazonS3,
	"googleCloudStorage": StreamSinkInTypeGoogleCloudStorage,
	"googleCloudPubSub":  StreamSinkInTypeGoogleCloudPubSub,
	"sqs":                StreamSinkInTypeSqs,
	"sns":                StreamSinkInTypeSns,
	"bigQuery":           StreamSinkInTypeBigQuery,
	"clickhouse":         StreamSinkInTypeClickhouse,
	"eventBridge":        StreamSinkInTypeEventBridge,
	"snowflake":          StreamSinkInTypeSnowflake,
	"rabbitMq":           StreamSinkInTypeRabbitMq,
	"redshift":           StreamSinkInTypeRedshift,
}
