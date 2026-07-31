// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class StreamOut(
    /** The stream's ID. */
    val id: String,
    /** The stream's UID. */
    val uid: String? = null,
    /** The stream's name. */
    val name: String? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    val metadata: Map<String, String>,
)
