// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type EndpointDisabledTrigger string

const (
	ENDPOINTDISABLEDTRIGGER_MANUAL    EndpointDisabledTrigger = "manual"
	ENDPOINTDISABLEDTRIGGER_AUTOMATIC EndpointDisabledTrigger = "automatic"
)

var allowedEndpointDisabledTrigger = []EndpointDisabledTrigger{
	"manual",
	"automatic",
}

func (v *EndpointDisabledTrigger) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := EndpointDisabledTrigger(value)
	if slices.Contains(allowedEndpointDisabledTrigger, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid EndpointDisabledTrigger", value)

}

var EndpointDisabledTriggerFromString = map[string]EndpointDisabledTrigger{
	"manual":    ENDPOINTDISABLEDTRIGGER_MANUAL,
	"automatic": ENDPOINTDISABLEDTRIGGER_AUTOMATIC,
}
