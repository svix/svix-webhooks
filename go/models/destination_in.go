// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
)

// The destination's type and type-specific configuration.
// When creating an DestinationIn, use the appropriate config structure based on the Type:
//   - "pollingEndpoint": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfigIn
//   - "bigQuery": Use BigQueryConfigIn
//   - "clickhouse": Use ClickhouseConfigIn
//   - "eventBridge": Use EventBridgeConfigIn
//   - "fifoEndpoint": Use FifoEndpointConfigIn
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigIn
//   - "googleCloudStorage": Use GoogleCloudStorageConfigIn
//   - "otelTracing": Use OtelTracingConfigIn
//   - "postgres": Use PostgresConfigIn
//   - "rabbitMq": Use RabbitMqConfigIn
//   - "redshift": Use RedshiftConfigIn
//   - "amazonS3": Use S3ConfigIn
//   - "snowflake": Use SnowflakeConfigIn
//   - "sns": Use SnsConfigIn
//   - "sqs": Use SqsConfigIn
type DestinationIn struct {
	Uid *string `json:"uid,omitempty"` // An optional unique identifier for the destination.
	// Whether the destination will receive events.
	//
	// If the destination is `enabled`, events sent to the application will be dispatched to the destination in order.
	//
	// If the destination is `disabled`, events will not be dispatched until the destination is reenabled.
	Status    *SinkStatusIn `json:"status,omitempty"`
	BatchSize *uint16       `json:"batchSize,omitempty"` // How many events will be batched in a request to the destination.
	// How long to wait before a batch of events is sent, if the `batchSize` is not reached.
	//
	// For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after 10 seconds or 100 events, whichever comes first.
	//
	// Note that an empty batch is never sent to the destination.
	MaxWaitSecs *uint16             `json:"maxWaitSecs,omitempty"`
	EventTypes  []string            `json:"eventTypes,omitempty"` // A list of event types that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
	Channels    []string            `json:"channels,omitempty"`   // A list of channels that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
	Metadata    *map[string]string  `json:"metadata,omitempty"`
	Type        DestinationInType   `json:"type"`
	Config      DestinationInConfig `json:"config"`
}

type DestinationInType string

const (
	DestinationInTypePollingEndpoint    DestinationInType = "pollingEndpoint"
	DestinationInTypeAzureBlobStorage   DestinationInType = "azureBlobStorage"
	DestinationInTypeOtelTracing        DestinationInType = "otelTracing"
	DestinationInTypeFifoEndpoint       DestinationInType = "fifoEndpoint"
	DestinationInTypeAmazonS3           DestinationInType = "amazonS3"
	DestinationInTypeGoogleCloudStorage DestinationInType = "googleCloudStorage"
	DestinationInTypeGoogleCloudPubSub  DestinationInType = "googleCloudPubSub"
	DestinationInTypeSqs                DestinationInType = "sqs"
	DestinationInTypeSns                DestinationInType = "sns"
	DestinationInTypeBigQuery           DestinationInType = "bigQuery"
	DestinationInTypeClickhouse         DestinationInType = "clickhouse"
	DestinationInTypeEventBridge        DestinationInType = "eventBridge"
	DestinationInTypeSnowflake          DestinationInType = "snowflake"
	DestinationInTypeRabbitMq           DestinationInType = "rabbitMq"
	DestinationInTypeRedshift           DestinationInType = "redshift"
	DestinationInTypePostgres           DestinationInType = "postgres"
)

type DestinationInConfig interface {
	isDestinationInConfig()
}

func (emptyMap) isDestinationInConfig()                   {}
func (AzureBlobStorageConfigIn) isDestinationInConfig()   {}
func (OtelTracingConfigIn) isDestinationInConfig()        {}
func (FifoEndpointConfigIn) isDestinationInConfig()       {}
func (S3ConfigIn) isDestinationInConfig()                 {}
func (GoogleCloudStorageConfigIn) isDestinationInConfig() {}
func (GoogleCloudPubSubConfigIn) isDestinationInConfig()  {}
func (SqsConfigIn) isDestinationInConfig()                {}
func (SnsConfigIn) isDestinationInConfig()                {}
func (BigQueryConfigIn) isDestinationInConfig()           {}
func (ClickhouseConfigIn) isDestinationInConfig()         {}
func (EventBridgeConfigIn) isDestinationInConfig()        {}
func (SnowflakeConfigIn) isDestinationInConfig()          {}
func (RabbitMqConfigIn) isDestinationInConfig()           {}
func (RedshiftConfigIn) isDestinationInConfig()           {}
func (PostgresConfigIn) isDestinationInConfig()           {}

func (i *DestinationIn) UnmarshalJSON(data []byte) error {
	type Alias DestinationIn
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
	case "fifoEndpoint":
		var c FifoEndpointConfigIn
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
	case "otelTracing":
		var c OtelTracingConfigIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "postgres":
		var c PostgresConfigIn
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

var DestinationInTypeWithNoConfig = map[string]bool{
	"pollingEndpoint": true,
}

func (i DestinationIn) MarshalJSON() ([]byte, error) {
	type Alias DestinationIn
	if _, found := DestinationInTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var DestinationInTypeFromString = map[string]DestinationInType{
	"pollingEndpoint":    DestinationInTypePollingEndpoint,
	"azureBlobStorage":   DestinationInTypeAzureBlobStorage,
	"otelTracing":        DestinationInTypeOtelTracing,
	"fifoEndpoint":       DestinationInTypeFifoEndpoint,
	"amazonS3":           DestinationInTypeAmazonS3,
	"googleCloudStorage": DestinationInTypeGoogleCloudStorage,
	"googleCloudPubSub":  DestinationInTypeGoogleCloudPubSub,
	"sqs":                DestinationInTypeSqs,
	"sns":                DestinationInTypeSns,
	"bigQuery":           DestinationInTypeBigQuery,
	"clickhouse":         DestinationInTypeClickhouse,
	"eventBridge":        DestinationInTypeEventBridge,
	"snowflake":          DestinationInTypeSnowflake,
	"rabbitMq":           DestinationInTypeRabbitMq,
	"redshift":           DestinationInTypeRedshift,
	"postgres":           DestinationInTypePostgres,
}
