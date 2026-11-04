// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type DestinationStatus string

const (
	DESTINATIONSTATUS_ENABLED  DestinationStatus = "enabled"
	DESTINATIONSTATUS_PAUSED   DestinationStatus = "paused"
	DESTINATIONSTATUS_DISABLED DestinationStatus = "disabled"
	DESTINATIONSTATUS_RETRYING DestinationStatus = "retrying"
)

var allowedDestinationStatus = []DestinationStatus{
	"enabled",
	"paused",
	"disabled",
	"retrying",
}

func (v *DestinationStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := DestinationStatus(value)
	if slices.Contains(allowedDestinationStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid DestinationStatus", value)

}

var DestinationStatusFromString = map[string]DestinationStatus{
	"enabled":  DESTINATIONSTATUS_ENABLED,
	"paused":   DESTINATIONSTATUS_PAUSED,
	"disabled": DESTINATIONSTATUS_DISABLED,
	"retrying": DESTINATIONSTATUS_RETRYING,
}
