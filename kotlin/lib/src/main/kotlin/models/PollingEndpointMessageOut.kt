// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class PollingEndpointMessageOut(
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    val headers: Map<String, String>? = null,
    val id: String,
    @Serializable(with = StringAnyMapSerializer::class) val payload: Map<String, Any>,
    val tags: Set<String>? = null,
    val timestamp: Instant,
)
