// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class StreamEventTypeOut(
    /** The event type's name */
    val name: String,
    val description: String? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deprecated: Boolean,
    val archived: Boolean,
    val featureFlags: Set<String>? = null,
)
