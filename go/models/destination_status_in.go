// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type DestinationStatusIn string

const (
	DESTINATIONSTATUSIN_ENABLED  DestinationStatusIn = "enabled"
	DESTINATIONSTATUSIN_DISABLED DestinationStatusIn = "disabled"
)

var allowedDestinationStatusIn = []DestinationStatusIn{
	"enabled",
	"disabled",
}

func (v *DestinationStatusIn) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := DestinationStatusIn(value)
	if slices.Contains(allowedDestinationStatusIn, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid DestinationStatusIn", value)

}

var DestinationStatusInFromString = map[string]DestinationStatusIn{
	"enabled":  DESTINATIONSTATUSIN_ENABLED,
	"disabled": DESTINATIONSTATUSIN_DISABLED,
}
