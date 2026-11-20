// Package svix this file is @generated DO NOT EDIT
package models

type ClientSecretJwtParamsIn struct {
	SecretBase64     string                   `json:"secretBase64"`     // The base64-encoded secret used for signing the JWT.
	SigningAlgorithm OauthJwsSigningAlgorithm `json:"signingAlgorithm"` // The algorithm used to sign the JWT.
	// Optional secret identifier.
	//
	// If supplied, this will be populated in the JWT header in the `kid` field.
	SecretId *string `json:"secretId,omitempty"`
	// Optional number of seconds after which the JWT should expire.
	//
	// Defaults to 300 seconds.
	TokenExpirySecs *uint64 `json:"tokenExpirySecs,omitempty"`
}
