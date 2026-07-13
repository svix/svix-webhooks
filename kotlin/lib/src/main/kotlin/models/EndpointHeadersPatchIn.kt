// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointHeadersPatchIn(
    val headers: Map<String, String>,
    /** A list of headers be be removed */
    val deleteHeaders: List<String>? = null,
)
