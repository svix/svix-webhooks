// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeFromOpenApi(
    /** The event type's name */
    val name: String,
    val description: String,
    @Serializable(with = StringAnyMapSerializer::class) val schemas: Map<String, Any>? = null,
    val deprecated: Boolean,
    /** The event type group's name */
    val groupName: String? = null,
    val featureFlags: Set<String>? = null,
)
