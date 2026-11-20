// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type Oauth2GrantTypeInOut string

const (
	OAUTH2GRANTTYPEINOUT_CLIENT_CREDENTIALS Oauth2GrantTypeInOut = "clientCredentials"
	OAUTH2GRANTTYPEINOUT_REFRESH_TOKEN      Oauth2GrantTypeInOut = "refreshToken"
)

var allowedOauth2GrantTypeInOut = []Oauth2GrantTypeInOut{
	"clientCredentials",
	"refreshToken",
}

func (v *Oauth2GrantTypeInOut) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := Oauth2GrantTypeInOut(value)
	if slices.Contains(allowedOauth2GrantTypeInOut, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid Oauth2GrantTypeInOut", value)

}

var Oauth2GrantTypeInOutFromString = map[string]Oauth2GrantTypeInOut{
	"clientCredentials": OAUTH2GRANTTYPEINOUT_CLIENT_CREDENTIALS,
	"refreshToken":      OAUTH2GRANTTYPEINOUT_REFRESH_TOKEN,
}
