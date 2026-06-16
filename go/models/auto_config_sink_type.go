// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
)

// When creating an AutoConfigSinkType, use the appropriate config structure based on the Type:
//   - "http": Use EndpointIn
//   - "poller": Use SinkInCommon
type AutoConfigSinkType struct {
	Type   AutoConfigSinkTypeType   `json:"type"`
	Config AutoConfigSinkTypeConfig `json:"config"`
}

type AutoConfigSinkTypeType string

const (
	AutoConfigSinkTypeTypePoller AutoConfigSinkTypeType = "poller"
	AutoConfigSinkTypeTypeHttp   AutoConfigSinkTypeType = "http"
)

type AutoConfigSinkTypeConfig interface {
	isAutoConfigSinkTypeConfig()
}

func (SinkInCommon) isAutoConfigSinkTypeConfig() {}
func (EndpointIn) isAutoConfigSinkTypeConfig()   {}

func (i *AutoConfigSinkType) UnmarshalJSON(data []byte) error {
	type Alias AutoConfigSinkType
	aux := struct {
		*Alias
		Config json.RawMessage `json:"config"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Type {
	case "http":
		var c EndpointIn
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "poller":
		var c SinkInCommon
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

func (i AutoConfigSinkType) MarshalJSON() ([]byte, error) {
	type Alias AutoConfigSinkType
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var AutoConfigSinkTypeTypeFromString = map[string]AutoConfigSinkTypeType{
	"poller": AutoConfigSinkTypeTypePoller,
	"http":   AutoConfigSinkTypeTypeHttp,
}
