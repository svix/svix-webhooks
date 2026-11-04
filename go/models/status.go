// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type Status string

const (
	STATUS_PENDING Status = "pending"
	STATUS_ACTIVE  Status = "active"
)

var allowedStatus = []Status{
	"pending",
	"active",
}

func (v *Status) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := Status(value)
	if slices.Contains(allowedStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid Status", value)

}

var StatusFromString = map[string]Status{
	"pending": STATUS_PENDING,
	"active":  STATUS_ACTIVE,
}
