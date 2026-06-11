// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type StartingPosition string

const (
	STARTINGPOSITION_EARLIEST StartingPosition = "earliest"
	STARTINGPOSITION_LATEST   StartingPosition = "latest"
)

var allowedStartingPosition = []StartingPosition{
	"earliest",
	"latest",
}

func (v *StartingPosition) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := StartingPosition(value)
	if slices.Contains(allowedStartingPosition, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid StartingPosition", value)

}

var StartingPositionFromString = map[string]StartingPosition{
	"earliest": STARTINGPOSITION_EARLIEST,
	"latest":   STARTINGPOSITION_LATEST,
}
