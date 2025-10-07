// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ApplicationAlertEvent string

const (
	APPLICATIONALERTEVENT_ENDPOINT_DISABLED         ApplicationAlertEvent = "endpoint.disabled"
	APPLICATIONALERTEVENT_MESSAGE_ATTEMPT_EXHAUSTED ApplicationAlertEvent = "message.attempt.exhausted"
	APPLICATIONALERTEVENT_STREAM_SINK_DISABLED      ApplicationAlertEvent = "stream.sink.disabled"
)

var allowedApplicationAlertEvent = []ApplicationAlertEvent{
	"endpoint.disabled",
	"message.attempt.exhausted",
	"stream.sink.disabled",
}

func (v *ApplicationAlertEvent) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ApplicationAlertEvent(value)
	if slices.Contains(allowedApplicationAlertEvent, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ApplicationAlertEvent", value)

}

var ApplicationAlertEventFromString = map[string]ApplicationAlertEvent{
	"endpoint.disabled":         APPLICATIONALERTEVENT_ENDPOINT_DISABLED,
	"message.attempt.exhausted": APPLICATIONALERTEVENT_MESSAGE_ATTEMPT_EXHAUSTED,
	"stream.sink.disabled":      APPLICATIONALERTEVENT_STREAM_SINK_DISABLED,
}
