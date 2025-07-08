// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeFromOpenApi(
    val deprecated: Boolean,
    val description: String,
    val featureFlag: String? = null,
    val featureFlags: Set<String>? = null,
    /** The event type group's name */
    val groupName: String? = null,
    /** The event type's name */
    val name: String,
    @Serializable(with = StringAnyMapSerializer::class) val schemas: Map<String, Any>? = null,
)
