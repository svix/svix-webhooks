// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeUpdate(
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    val description: String,
    val featureFlag: String? = null,
    /** The event type group's name */
    val groupName: String? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    /** The schema for the event type for a specific version as a JSON schema. */
    val schemas: Map<String, Any>? = null,
)
