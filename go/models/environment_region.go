// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type EnvironmentRegion string

const (
	ENVIRONMENTREGION_EU          EnvironmentRegion = "eu"
	ENVIRONMENTREGION_US          EnvironmentRegion = "us"
	ENVIRONMENTREGION_IN          EnvironmentRegion = "in"
	ENVIRONMENTREGION_AU          EnvironmentRegion = "au"
	ENVIRONMENTREGION_CA          EnvironmentRegion = "ca"
	ENVIRONMENTREGION_SELF_HOSTED EnvironmentRegion = "self-hosted"
)

var allowedEnvironmentRegion = []EnvironmentRegion{
	"eu",
	"us",
	"in",
	"au",
	"ca",
	"self-hosted",
}

func (v *EnvironmentRegion) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := EnvironmentRegion(value)
	if slices.Contains(allowedEnvironmentRegion, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid EnvironmentRegion", value)

}

var EnvironmentRegionFromString = map[string]EnvironmentRegion{
	"eu":          ENVIRONMENTREGION_EU,
	"us":          ENVIRONMENTREGION_US,
	"in":          ENVIRONMENTREGION_IN,
	"au":          ENVIRONMENTREGION_AU,
	"ca":          ENVIRONMENTREGION_CA,
	"self-hosted": ENVIRONMENTREGION_SELF_HOSTED,
}
