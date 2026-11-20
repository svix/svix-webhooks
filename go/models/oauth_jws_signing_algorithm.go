// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type OauthJwsSigningAlgorithm string

const (
	OAUTHJWSSIGNINGALGORITHM_RS256 OauthJwsSigningAlgorithm = "RS256"
)

var allowedOauthJwsSigningAlgorithm = []OauthJwsSigningAlgorithm{
	"RS256",
}

func (v *OauthJwsSigningAlgorithm) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := OauthJwsSigningAlgorithm(value)
	if slices.Contains(allowedOauthJwsSigningAlgorithm, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid OauthJwsSigningAlgorithm", value)

}

var OauthJwsSigningAlgorithmFromString = map[string]OauthJwsSigningAlgorithm{
	"RS256": OAUTHJWSSIGNINGALGORITHM_RS256,
}
