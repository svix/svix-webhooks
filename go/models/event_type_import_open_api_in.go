// Package svix this file is @generated DO NOT EDIT
package models

// Import a list of event types from webhooks defined in an OpenAPI spec.
//
// The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
type EventTypeImportOpenApiIn struct {
	// If `true`, return the event types that would be modified without actually modifying them.
	DryRun *bool `json:"dryRun,omitempty"`
	// If `true`, all existing event types that are not in the spec will be archived.
	ReplaceAll *bool `json:"replaceAll,omitempty"`
	// A pre-parsed JSON spec.
	Spec *map[string]interface{} `json:"spec,omitempty"`
	// A string, parsed by the server as YAML or JSON.
	SpecRaw *string `json:"specRaw,omitempty"`
}
