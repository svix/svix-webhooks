// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeOut(
    /** The event type's name */
    val name: String,
    val description: String,
    val archived: Boolean? = null,
    val deprecated: Boolean,
    @Serializable(with = StringAnyMapSerializer::class)
    /** The schema for the event type for a specific version as a JSON schema. */
    val schemas: Map<String, Any>? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    /** The event type group's name */
    val groupName: String? = null,
    val featureFlags: Set<String>? = null,
    val featureFlag: String? = null,
)
