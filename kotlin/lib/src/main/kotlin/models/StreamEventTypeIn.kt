// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class StreamEventTypeIn(
    /** The event type's name */
    val name: String,
    val description: String? = null,
    val featureFlags: Set<String>? = null,
    val deprecated: Boolean? = null,
    val archived: Boolean? = null,
)
