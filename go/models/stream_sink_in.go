// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
)

// When creating an StreamSinkIn, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "azureBlobStorage": Use AzureBlobStorageConfig
//   - "googleCloudStorage": Use GoogleCloudStorageConfig
//   - "amazonS3": Use S3Config
//   - "http": Use SinkHttpConfig
//   - "otelTracing": Use SinkOtelV1Config
type StreamSinkIn struct {
	BatchSize  *uint16  `json:"batchSize,omitempty"`  // How many events will be batched in a request to the Sink.
	EventTypes []string `json:"eventTypes,omitempty"` // A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events.
	// How long to wait before a batch of events is sent, if the `batchSize` is not reached.
	//
	// For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.
	//
	// Note that we will never send an empty batch of events to the Sink.
	MaxWaitSecs *uint16            `json:"maxWaitSecs,omitempty"`
	Metadata    *map[string]string `json:"metadata,omitempty"`
	// Whether the sink will receive events.
	//
	// If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.
	//
	// If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled.
	Status *SinkStatusIn      `json:"status,omitempty"`
	Uid    *string            `json:"uid,omitempty"` // An optional unique identifier for the sink.
	Type   StreamSinkInType   `json:"type"`
	Config StreamSinkInConfig `json:"config"`
}

type StreamSinkInType string

const (
	StreamSinkInTypePoller             StreamSinkInType = "poller"
	StreamSinkInTypeAzureBlobStorage   StreamSinkInType = "azureBlobStorage"
	StreamSinkInTypeOtelTracing        StreamSinkInType = "otelTracing"
	StreamSinkInTypeHttp               StreamSinkInType = "http"
	StreamSinkInTypeAmazonS3           StreamSinkInType = "amazonS3"
	StreamSinkInTypeGoogleCloudStorage StreamSinkInType = "googleCloudStorage"
)

type StreamSinkInConfig interface {
	isStreamSinkInConfig()
}

func (emptyMap) isStreamSinkInConfig()                 {}
func (AzureBlobStorageConfig) isStreamSinkInConfig()   {}
func (SinkOtelV1Config) isStreamSinkInConfig()         {}
func (SinkHttpConfig) isStreamSinkInConfig()           {}
func (S3Config) isStreamSinkInConfig()                 {}
func (GoogleCloudStorageConfig) isStreamSinkInConfig() {}

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
}
