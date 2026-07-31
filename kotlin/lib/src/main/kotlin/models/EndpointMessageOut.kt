// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EndpointMessageOut(
    val status: MessageStatus,
    val statusText: MessageStatusText,
    val nextAttempt: Instant? = null,
    /** Optional unique identifier for the message */
    val eventId: String? = null,
    /** The event type's name */
    val eventType: String,
    @Serializable(with = StringAnyMapSerializer::class) val payload: Map<String, Any>,
    /** List of free-form identifiers that endpoints can filter by */
    val channels: Set<String>? = null,
    /** The Message's ID. */
    val id: String,
    val timestamp: Instant,
    val tags: Set<String>? = null,
    val deliverAt: Instant? = null,
)
