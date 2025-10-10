// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/utils"
)

// When creating an StreamSinkPatch, use the appropriate config structure based on the Type:
//   - "poller": No config needed (nil or just ignore the config field)
//   - "amazonS3": Use AmazonS3PatchConfig
//   - "azureBlobStorage": Use AzureBlobStoragePatchConfig
//   - "googleCloudStorage": Use GoogleCloudStoragePatchConfig
//   - "http": Use HttpPatchConfig
//   - "otelTracing": Use OtelTracingPatchConfig
type StreamSinkPatch struct {
	BatchSize   utils.Nullable[uint16]       `json:"batchSize"`
	EventTypes  []string                     `json:"eventTypes,omitempty"`
	MaxWaitSecs utils.Nullable[uint16]       `json:"maxWaitSecs"`
	Metadata    *map[string]string           `json:"metadata,omitempty"`
	Status      utils.Nullable[SinkStatusIn] `json:"status"`
	Uid         utils.Nullable[string]       `json:"uid"` // The StreamSink's UID.
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
)

type StreamSinkPatchConfig interface {
	isStreamSinkPatchConfig()
}

func (emptyMap) isStreamSinkPatchConfig()                      {}
func (AzureBlobStoragePatchConfig) isStreamSinkPatchConfig()   {}
func (OtelTracingPatchConfig) isStreamSinkPatchConfig()        {}
func (HttpPatchConfig) isStreamSinkPatchConfig()               {}
func (AmazonS3PatchConfig) isStreamSinkPatchConfig()           {}
func (GoogleCloudStoragePatchConfig) isStreamSinkPatchConfig() {}

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
		var c AmazonS3PatchConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "azureBlobStorage":
		var c AzureBlobStoragePatchConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "googleCloudStorage":
		var c GoogleCloudStoragePatchConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "http":
		var c HttpPatchConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "otelTracing":
		var c OtelTracingPatchConfig
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
}
