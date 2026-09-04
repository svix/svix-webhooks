// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"time"
)

// Equivalent to [`SinkConfigOut`], with `fifoEndpoint` instead of `http`.
// When creating an DestinationOut, use the appropriate config structure based on the Type:
//   - "pollingEndpoint": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfigOut
//   - "bigQuery": Use BigQueryConfigOut
//   - "clickhouse": Use ClickhouseConfigOut
//   - "eventBridge": Use EventBridgeConfigOut
//   - "googleCloudPubSub": Use GoogleCloudPubSubConfigOut
//   - "googleCloudStorage": Use GoogleCloudStorageConfigOut
//   - "otelTracing": Use OtelTracingConfigOut
//   - "postgres": Use PostgresConfigOut
//   - "rabbitMq": Use RabbitMqConfigOut
//   - "redshift": Use RedshiftConfigOut
//   - "amazonS3": Use S3ConfigOut
//   - "fifoEndpoint": Use SinkHttpConfigOut
//   - "snowflake": Use SnowflakeConfigOut
//   - "sns": Use SnsConfigOut
//   - "sqs": Use SqsConfigOut
type DestinationOut struct {
	Id              string               `json:"id"`            // The destination's ID.
	Uid             *string              `json:"uid,omitempty"` // The destination's UID.
	Status          SinkStatus           `json:"status"`
	CurrentIterator string               `json:"currentIterator"`
	FailureReason   *string              `json:"failureReason,omitempty"`
	CreatedAt       time.Time            `json:"createdAt"`
	UpdatedAt       time.Time            `json:"updatedAt"`
	BatchSize       int32                `json:"batchSize"`
	MaxWaitSecs     int32                `json:"maxWaitSecs"`
	EventTypes      []string             `json:"eventTypes,omitempty"`
	Channels        []string             `json:"channels,omitempty"`
	NextRetryAt     *time.Time           `json:"nextRetryAt,omitempty"`
	Metadata        map[string]string    `json:"metadata"`
	Type            DestinationOutType   `json:"type"`
	Config          DestinationOutConfig `json:"config"`
}

type DestinationOutType string

const (
	DestinationOutTypePollingEndpoint    DestinationOutType = "pollingEndpoint"
	DestinationOutTypeAzureBlobStorage   DestinationOutType = "azureBlobStorage"
	DestinationOutTypeOtelTracing        DestinationOutType = "otelTracing"
	DestinationOutTypeFifoEndpoint       DestinationOutType = "fifoEndpoint"
	DestinationOutTypeAmazonS3           DestinationOutType = "amazonS3"
	DestinationOutTypeSnowflake          DestinationOutType = "snowflake"
	DestinationOutTypeGoogleCloudStorage DestinationOutType = "googleCloudStorage"
	DestinationOutTypeGoogleCloudPubSub  DestinationOutType = "googleCloudPubSub"
	DestinationOutTypeRedshift           DestinationOutType = "redshift"
	DestinationOutTypeBigQuery           DestinationOutType = "bigQuery"
	DestinationOutTypeClickhouse         DestinationOutType = "clickhouse"
	DestinationOutTypeRabbitMq           DestinationOutType = "rabbitMq"
	DestinationOutTypeSqs                DestinationOutType = "sqs"
	DestinationOutTypeEventBridge        DestinationOutType = "eventBridge"
	DestinationOutTypeSns                DestinationOutType = "sns"
	DestinationOutTypePostgres           DestinationOutType = "postgres"
)

type DestinationOutConfig interface {
	isDestinationOutConfig()
}

func (emptyMap) isDestinationOutConfig()                    {}
func (AzureBlobStorageConfigOut) isDestinationOutConfig()   {}
func (OtelTracingConfigOut) isDestinationOutConfig()        {}
func (SinkHttpConfigOut) isDestinationOutConfig()           {}
func (S3ConfigOut) isDestinationOutConfig()                 {}
func (SnowflakeConfigOut) isDestinationOutConfig()          {}
func (GoogleCloudStorageConfigOut) isDestinationOutConfig() {}
func (GoogleCloudPubSubConfigOut) isDestinationOutConfig()  {}
func (RedshiftConfigOut) isDestinationOutConfig()           {}
func (BigQueryConfigOut) isDestinationOutConfig()           {}
func (ClickhouseConfigOut) isDestinationOutConfig()         {}
func (RabbitMqConfigOut) isDestinationOutConfig()           {}
func (SqsConfigOut) isDestinationOutConfig()                {}
func (EventBridgeConfigOut) isDestinationOutConfig()        {}
func (SnsConfigOut) isDestinationOutConfig()                {}
func (PostgresConfigOut) isDestinationOutConfig()           {}

func (i *DestinationOut) UnmarshalJSON(data []byte) error {
	type Alias DestinationOut
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
	case "postgres":
		var c PostgresConfigOut
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
	case "fifoEndpoint":
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

var DestinationOutTypeWithNoConfig = map[string]bool{
	"pollingEndpoint": true,
}

func (i DestinationOut) MarshalJSON() ([]byte, error) {
	type Alias DestinationOut
	if _, found := DestinationOutTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var DestinationOutTypeFromString = map[string]DestinationOutType{
	"pollingEndpoint":    DestinationOutTypePollingEndpoint,
	"azureBlobStorage":   DestinationOutTypeAzureBlobStorage,
	"otelTracing":        DestinationOutTypeOtelTracing,
	"fifoEndpoint":       DestinationOutTypeFifoEndpoint,
	"amazonS3":           DestinationOutTypeAmazonS3,
	"snowflake":          DestinationOutTypeSnowflake,
	"googleCloudStorage": DestinationOutTypeGoogleCloudStorage,
	"googleCloudPubSub":  DestinationOutTypeGoogleCloudPubSub,
	"redshift":           DestinationOutTypeRedshift,
	"bigQuery":           DestinationOutTypeBigQuery,
	"clickhouse":         DestinationOutTypeClickhouse,
	"rabbitMq":           DestinationOutTypeRabbitMq,
	"sqs":                DestinationOutTypeSqs,
	"eventBridge":        DestinationOutTypeEventBridge,
	"sns":                DestinationOutTypeSns,
	"postgres":           DestinationOutTypePostgres,
}
