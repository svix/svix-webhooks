// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class IngestEndpointUpdate(
    val description: String? = null,
    val disabled: Boolean? = null,
    val metadata: Map<String, String>? = null,
    val rateLimit: UShort? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val url: String,
)
