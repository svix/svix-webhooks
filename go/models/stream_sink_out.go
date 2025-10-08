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
//   - "googleCloudStorage": Use GoogleCloudStorageConfig
//   - "amazonS3": Use S3Config
//   - "http": Use SinkHttpConfig
//   - "otelTracing": Use SinkOtelV1Config
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
	case "googleCloudStorage":
		var c GoogleCloudStorageConfig
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
}
