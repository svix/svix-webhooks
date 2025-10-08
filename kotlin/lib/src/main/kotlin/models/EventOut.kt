// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EventOut(
    /** The event type's name */
    val eventType: String,
    val payload: String,
    val timestamp: Instant,
)
