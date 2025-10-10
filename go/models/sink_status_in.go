// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type SinkStatusIn string

const (
	SINKSTATUSIN_ENABLED  SinkStatusIn = "enabled"
	SINKSTATUSIN_DISABLED SinkStatusIn = "disabled"
)

var allowedSinkStatusIn = []SinkStatusIn{
	"enabled",
	"disabled",
}

func (v *SinkStatusIn) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := SinkStatusIn(value)
	if slices.Contains(allowedSinkStatusIn, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid SinkStatusIn", value)

}

var SinkStatusInFromString = map[string]SinkStatusIn{
	"enabled":  SINKSTATUSIN_ENABLED,
	"disabled": SINKSTATUSIN_DISABLED,
}
