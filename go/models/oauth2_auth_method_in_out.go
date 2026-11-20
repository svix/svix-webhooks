// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// The method used for authenticating to the OAuth authorization server.
//
// `clientSecretJwt` will construct a JWT used for authentication
// with the oauth authorization server. This method is less
// commonly used and may not be supported by all oauth providers.
// `clientSecretBasic` will authenticate to the oauth authorization server
// using an `Authorization` header with the client secret as the value.
// This is the most common means of authentication.
// `clientSecretPost` will authenticate to the oauth authorization server
// by passing the client secret in a `client_secret` field in the
// request body. This method may not be supported by all oauth providers,
// and in general `clientSecretBasic` should be preferred.
type Oauth2AuthMethodInOut string

const (
	OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_JWT   Oauth2AuthMethodInOut = "clientSecretJwt"
	OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_BASIC Oauth2AuthMethodInOut = "clientSecretBasic"
	OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_POST  Oauth2AuthMethodInOut = "clientSecretPost"
)

var allowedOauth2AuthMethodInOut = []Oauth2AuthMethodInOut{
	"clientSecretJwt",
	"clientSecretBasic",
	"clientSecretPost",
}

func (v *Oauth2AuthMethodInOut) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := Oauth2AuthMethodInOut(value)
	if slices.Contains(allowedOauth2AuthMethodInOut, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid Oauth2AuthMethodInOut", value)

}

var Oauth2AuthMethodInOutFromString = map[string]Oauth2AuthMethodInOut{
	"clientSecretJwt":   OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_JWT,
	"clientSecretBasic": OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_BASIC,
	"clientSecretPost":  OAUTH2AUTHMETHODINOUT_CLIENT_SECRET_POST,
}
