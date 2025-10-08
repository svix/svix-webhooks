// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class StreamEventTypeOut(
    val archived: Boolean,
    val createdAt: Instant,
    val deprecated: Boolean,
    val description: String? = null,
    val featureFlags: Set<String>? = null,
    /** The event type's name */
    val name: String,
    val updatedAt: Instant,
)
