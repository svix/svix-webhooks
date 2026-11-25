// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ApplicationAlertEvent string

const (
	APPLICATIONALERTEVENT_ENDPOINT_DISABLED             ApplicationAlertEvent = "endpoint.disabled"
	APPLICATIONALERTEVENT_MESSAGE_ATTEMPT_EXHAUSTED     ApplicationAlertEvent = "message.attempt.exhausted"
	APPLICATIONALERTEVENT_STREAM_SINK_DISABLED          ApplicationAlertEvent = "stream.sink.disabled"
	APPLICATIONALERTEVENT_ENDPOINT_RESPONSE_STATUS_CODE ApplicationAlertEvent = "endpoint.response.status_code"
)

var allowedApplicationAlertEvent = []ApplicationAlertEvent{
	"endpoint.disabled",
	"message.attempt.exhausted",
	"stream.sink.disabled",
	"endpoint.response.status_code",
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
	"endpoint.disabled":             APPLICATIONALERTEVENT_ENDPOINT_DISABLED,
	"message.attempt.exhausted":     APPLICATIONALERTEVENT_MESSAGE_ATTEMPT_EXHAUSTED,
	"stream.sink.disabled":          APPLICATIONALERTEVENT_STREAM_SINK_DISABLED,
	"endpoint.response.status_code": APPLICATIONALERTEVENT_ENDPOINT_RESPONSE_STATUS_CODE,
}
