// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class OperationalWebhookEndpointOut(
    /** The Endpoint's ID. */
    val id: String,
    val url: String,
    val description: String,
    /**
     * Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val disabled: Boolean? = null,
    val eventTypes: Set<String>? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
    val metadata: Map<String, String>,
)
