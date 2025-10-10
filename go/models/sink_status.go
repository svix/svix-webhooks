// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type SinkStatus string

const (
	SINKSTATUS_ENABLED  SinkStatus = "enabled"
	SINKSTATUS_PAUSED   SinkStatus = "paused"
	SINKSTATUS_DISABLED SinkStatus = "disabled"
	SINKSTATUS_RETRYING SinkStatus = "retrying"
)

var allowedSinkStatus = []SinkStatus{
	"enabled",
	"paused",
	"disabled",
	"retrying",
}

func (v *SinkStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := SinkStatus(value)
	if slices.Contains(allowedSinkStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid SinkStatus", value)

}

var SinkStatusFromString = map[string]SinkStatus{
	"enabled":  SINKSTATUS_ENABLED,
	"paused":   SINKSTATUS_PAUSED,
	"disabled": SINKSTATUS_DISABLED,
	"retrying": SINKSTATUS_RETRYING,
}
