// Package svix this file is @generated DO NOT EDIT
package models

type AppPortalAccessIn struct {
	// Optionally creates a new application while generating the access link.
	//
	// If the application id or uid that is used in the path already exists, this argument is ignored.
	Application *ApplicationIn `json:"application,omitempty"`
	// Custom capabilities attached to the token, You can combine as many capabilities as necessary.
	//
	// The `ViewBase` capability is always required
	//
	// - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint secret.
	//
	// - `ViewEndpointSecret`: Allows user to view the endpoint secret.
	//
	// - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.
	//
	// - `ManageTransformations`: Allows user to modify the endpoint transformations.
	//
	// - `CreateAttempts`: Allows user to replay missing messages and send example messages.
	//
	// - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint (including secrets)
	Capabilities []AppPortalCapability `json:"capabilities,omitempty"`
	// How long the token will be valid for, in seconds.
	//
	// Valid values are between 1 hour and 7 days. The default is 7 days.
	Expiry       *uint64  `json:"expiry,omitempty"`
	FeatureFlags []string `json:"featureFlags,omitempty"` // The set of feature flags the created token will have access to.
	ReadOnly     *bool    `json:"readOnly,omitempty"`     // Whether the app portal should be in read-only mode.
	// An optional session ID to attach to the token.
	//
	// When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
	SessionId *string `json:"sessionId,omitempty"`
}
