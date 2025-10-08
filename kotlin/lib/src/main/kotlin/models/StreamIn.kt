// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class StreamIn(
    val metadata: Map<String, String>? = null,
    /** The stream's name. */
    val name: String,
    /** An optional unique identifier for the stream. */
    val uid: String? = null,
)
