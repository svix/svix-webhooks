// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type McpEventsDeliveryMode string

const (
	MCPEVENTSDELIVERYMODE_WEBHOOK McpEventsDeliveryMode = "webhook"
)

var allowedMcpEventsDeliveryMode = []McpEventsDeliveryMode{
	"webhook",
}

func (v *McpEventsDeliveryMode) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := McpEventsDeliveryMode(value)
	if slices.Contains(allowedMcpEventsDeliveryMode, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid McpEventsDeliveryMode", value)

}

var McpEventsDeliveryModeFromString = map[string]McpEventsDeliveryMode{
	"webhook": MCPEVENTSDELIVERYMODE_WEBHOOK,
}
