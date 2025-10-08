// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class StreamEventTypeIn(
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    val description: String? = null,
    val featureFlags: Set<String>? = null,
    /** The event type's name */
    val name: String,
)
