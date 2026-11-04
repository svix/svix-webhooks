// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a PostHog sink.
type PostHogConfigIn struct {
	// The PostHog ingestion domain.
	//
	// This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
	// and your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
	// (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
	Domain string `json:"domain"`
	// The PostHog project API key.
	//
	// Sent as the `api_key` field in the body of each `/batch/` request.
	ProjectToken string `json:"projectToken"`
}
