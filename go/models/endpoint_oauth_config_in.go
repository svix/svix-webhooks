// Package svix this file is @generated DO NOT EDIT
package models

type EndpointOauthConfigIn struct {
	GrantType  Oauth2GrantTypeInOut     `json:"grantType"` // The OAuth grant type.
	AuthMethod Oauth2AuthMethodInOut    `json:"authMethod"`
	JwtParams  *ClientSecretJwtParamsIn `json:"jwtParams,omitempty"` // Optional JWT parameters. Only required for `clientSecretJwt`
	ClientId   string                   `json:"clientId"`            // The client ID. Required for all authentication types.
	// Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
	//
	// For `clientSecretBasic`, the secret will be appended to the `Authorization` header.
	// For `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
	ClientSecret *string            `json:"clientSecret,omitempty"`
	TokenUrl     string             `json:"tokenUrl"`               // The URL of the authorization server.
	Scopes       []string           `json:"scopes,omitempty"`       // Optional OAuth scopes added to the request body.
	ExtraParams  *map[string]string `json:"extraParams,omitempty"`  // Extra parameters added to the request body as key-value pairs.
	RefreshToken *string            `json:"refreshToken,omitempty"` // For `refreshToken` grant type.
	ExtraHeaders *map[string]string `json:"extraHeaders,omitempty"` // Optional headers added to the token request.
}
