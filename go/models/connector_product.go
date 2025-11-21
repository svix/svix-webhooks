// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ConnectorProduct string

const (
	CONNECTORPRODUCT_DISPATCH ConnectorProduct = "Dispatch"
	CONNECTORPRODUCT_STREAM   ConnectorProduct = "Stream"
)

var allowedConnectorProduct = []ConnectorProduct{
	"Dispatch",
	"Stream",
}

func (v *ConnectorProduct) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ConnectorProduct(value)
	if slices.Contains(allowedConnectorProduct, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ConnectorProduct", value)

}

var ConnectorProductFromString = map[string]ConnectorProduct{
	"Dispatch": CONNECTORPRODUCT_DISPATCH,
	"Stream":   CONNECTORPRODUCT_STREAM,
}
