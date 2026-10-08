// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining an ElasticSearch/OpenSearch sink.
type ElasticSearchConfigIn struct {
	// Name of the index to write to
	//
	// This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch
	IndexName string `json:"indexName"`
	// Base URL to send indexing requests
	//
	// This should not include the /{index}/_bulk suffix.
	Url string `json:"url"`
	// API key for authentication for ElasticSearch
	//
	// If not passed, any username:password embedded in the URL will be used. If none is passed,
	// the indexing will be done unauthenticated.
	ApiKey  *string `json:"apiKey,omitempty"`
	Refresh *bool   `json:"refresh,omitempty"` // If true, Elasticsearch refreshes the affected shards to make this operation visible to search.
}
