// Package svix this file is @generated DO NOT EDIT
package models

type EndpointOauthConfigOut struct {
	GrantType    Oauth2GrantTypeInOut  `json:"grantType"`
	AuthMethod   Oauth2AuthMethodInOut `json:"authMethod"`
	ClientId     string                `json:"clientId"`
	TokenUrl     string                `json:"tokenUrl"`
	Scopes       []string              `json:"scopes,omitempty"`
	ExtraParams  *map[string]string    `json:"extraParams,omitempty"`
	ExtraHeaders *map[string]string    `json:"extraHeaders,omitempty"`
}
