// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class StreamOut(
    val createdAt: Instant,
    /** The stream's ID. */
    val id: String,
    val metadata: Map<String, String>,
    /** The stream's name. */
    val name: String? = null,
    /** The stream's UID. */
    val uid: String? = null,
    val updatedAt: Instant,
)
