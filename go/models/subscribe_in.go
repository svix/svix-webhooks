package models

// SubscribeIn is the request body for updating an auto-config endpoint.
// Hand-maintained to match OpenAPI until this schema is included in full codegen output.
type SubscribeIn struct {
	Endpoint EndpointIn `json:"endpoint"`
}

func NewSubscribeIn(endpoint EndpointIn) SubscribeIn {
	return SubscribeIn{Endpoint: endpoint}
}
