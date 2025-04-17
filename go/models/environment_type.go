// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type EnvironmentType string

const (
	ENVIRONMENTTYPE_DEVELOPMENT EnvironmentType = "development"
	ENVIRONMENTTYPE_PRODUCTION  EnvironmentType = "production"
)

var allowedEnvironmentType = []EnvironmentType{
	"development",
	"production",
}

func (v *EnvironmentType) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := EnvironmentType(value)
	if slices.Contains(allowedEnvironmentType, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid EnvironmentType", value)

}

var EnvironmentTypeFromString = map[string]EnvironmentType{
	"development": ENVIRONMENTTYPE_DEVELOPMENT,
	"production":  ENVIRONMENTTYPE_PRODUCTION,
}
