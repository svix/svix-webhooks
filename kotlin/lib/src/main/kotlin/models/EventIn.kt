// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventIn(
    /** The event type's name */
    val eventType: String,
    val payload: String,
)
