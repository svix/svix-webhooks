// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a Segment sink.
type SegmentSinkConfigIn struct {
	// The Segment ingestion host.
	//
	// This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
	// for EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint
	// under this host.
	Host string `json:"host"`
	// The Segment source write key.
	//
	// Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
	// `/v1/batch` request.
	WriteKey string `json:"writeKey"`
}
