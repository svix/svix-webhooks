// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ElasticSearchConfigIn(
    /**
     * Name of the index to write to
     *
     * This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch
     */
    val indexName: String,
    /**
     * Base URL to send indexing requests
     *
     * This should not include the /{index}/_bulk suffix.
     */
    val url: String,
    /**
     * API key for authentication for ElasticSearch
     *
     * If not passed, any username:password embedded in the URL will be used. If none is passed, the
     * indexing will be done unauthenticated.
     */
    val apiKey: String? = null,
    /**
     * If true, Elasticsearch refreshes the affected shards to make this operation visible to
     * search.
     */
    val refresh: Boolean? = null,
)
