// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ElasticSearchConfigPatch(
    val indexName: String? = null,
    val url: String? = null,
    val apiKey: String? = null,
    val refresh: Boolean? = null,
)
