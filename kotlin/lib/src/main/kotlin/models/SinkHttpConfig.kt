// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SinkHttpConfig(
    val url: String,
    val headers: Map<String, String>? = null,
    val key: String? = null,
)
