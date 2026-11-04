// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ArchiveStatus string

const (
	ARCHIVESTATUS_ACTIVE   ArchiveStatus = "active"
	ARCHIVESTATUS_ARCHIVED ArchiveStatus = "archived"
	ARCHIVESTATUS_ALL      ArchiveStatus = "all"
)

var allowedArchiveStatus = []ArchiveStatus{
	"active",
	"archived",
	"all",
}

func (v *ArchiveStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ArchiveStatus(value)
	if slices.Contains(allowedArchiveStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ArchiveStatus", value)

}

var ArchiveStatusFromString = map[string]ArchiveStatus{
	"active":   ARCHIVESTATUS_ACTIVE,
	"archived": ARCHIVESTATUS_ARCHIVED,
	"all":      ARCHIVESTATUS_ALL,
}
