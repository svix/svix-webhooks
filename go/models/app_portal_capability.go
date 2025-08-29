// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type AppPortalCapability string

const (
	APPPORTALCAPABILITY_VIEW_BASE              AppPortalCapability = "ViewBase"
	APPPORTALCAPABILITY_VIEW_ENDPOINT_SECRET   AppPortalCapability = "ViewEndpointSecret"
	APPPORTALCAPABILITY_MANAGE_ENDPOINT_SECRET AppPortalCapability = "ManageEndpointSecret"
	APPPORTALCAPABILITY_MANAGE_TRANSFORMATIONS AppPortalCapability = "ManageTransformations"
	APPPORTALCAPABILITY_CREATE_ATTEMPTS        AppPortalCapability = "CreateAttempts"
	APPPORTALCAPABILITY_MANAGE_ENDPOINT        AppPortalCapability = "ManageEndpoint"
)

var allowedAppPortalCapability = []AppPortalCapability{
	"ViewBase",
	"ViewEndpointSecret",
	"ManageEndpointSecret",
	"ManageTransformations",
	"CreateAttempts",
	"ManageEndpoint",
}

func (v *AppPortalCapability) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := AppPortalCapability(value)
	if slices.Contains(allowedAppPortalCapability, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid AppPortalCapability", value)

}

var AppPortalCapabilityFromString = map[string]AppPortalCapability{
	"ViewBase":              APPPORTALCAPABILITY_VIEW_BASE,
	"ViewEndpointSecret":    APPPORTALCAPABILITY_VIEW_ENDPOINT_SECRET,
	"ManageEndpointSecret":  APPPORTALCAPABILITY_MANAGE_ENDPOINT_SECRET,
	"ManageTransformations": APPPORTALCAPABILITY_MANAGE_TRANSFORMATIONS,
	"CreateAttempts":        APPPORTALCAPABILITY_CREATE_ATTEMPTS,
	"ManageEndpoint":        APPPORTALCAPABILITY_MANAGE_ENDPOINT,
}
