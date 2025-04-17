// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type BorderRadiusEnum string

const (
	BORDERRADIUSENUM_NONE BorderRadiusEnum = "none"
	BORDERRADIUSENUM_LG   BorderRadiusEnum = "lg"
	BORDERRADIUSENUM_MD   BorderRadiusEnum = "md"
	BORDERRADIUSENUM_SM   BorderRadiusEnum = "sm"
	BORDERRADIUSENUM_FULL BorderRadiusEnum = "full"
)

var allowedBorderRadiusEnum = []BorderRadiusEnum{
	"none",
	"lg",
	"md",
	"sm",
	"full",
}

func (v *BorderRadiusEnum) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := BorderRadiusEnum(value)
	if slices.Contains(allowedBorderRadiusEnum, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid BorderRadiusEnum", value)

}

var BorderRadiusEnumFromString = map[string]BorderRadiusEnum{
	"none": BORDERRADIUSENUM_NONE,
	"lg":   BORDERRADIUSENUM_LG,
	"md":   BORDERRADIUSENUM_MD,
	"sm":   BORDERRADIUSENUM_SM,
	"full": BORDERRADIUSENUM_FULL,
}
