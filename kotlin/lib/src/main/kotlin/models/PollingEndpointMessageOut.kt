// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class PollingEndpointMessageOut(
    /** List of free-form identifiers that endpoints can filter by */
    val channels: Set<String>? = null,
    val deliverAt: Instant? = null,
    /** Optional unique identifier for the message */
    val eventId: String? = null,
    /** The event type's name */
    val eventType: String,
    val headers: Map<String, String>? = null,
    /** The Message's ID. */
    val id: String,
    @Serializable(with = StringAnyMapSerializer::class) val payload: Map<String, Any>,
    val tags: Set<String>? = null,
    val timestamp: Instant,
)
