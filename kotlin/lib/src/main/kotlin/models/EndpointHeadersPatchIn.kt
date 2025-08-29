// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointHeadersPatchIn(
    /** A list of headers be be removed */
    val deleteHeaders: List<String>? = null,
    val headers: Map<String, String>,
)
