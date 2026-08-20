// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type BulkExpungeStatus string

const (
	BULKEXPUNGESTATUS_EXPUNGED  BulkExpungeStatus = "expunged"
	BULKEXPUNGESTATUS_NOT_FOUND BulkExpungeStatus = "not-found"
)

var allowedBulkExpungeStatus = []BulkExpungeStatus{
	"expunged",
	"not-found",
}

func (v *BulkExpungeStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := BulkExpungeStatus(value)
	if slices.Contains(allowedBulkExpungeStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid BulkExpungeStatus", value)

}

var BulkExpungeStatusFromString = map[string]BulkExpungeStatus{
	"expunged":  BULKEXPUNGESTATUS_EXPUNGED,
	"not-found": BULKEXPUNGESTATUS_NOT_FOUND,
}
