// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageEndpointOut(
    /** The Endpoint's ID. */
    val id: String,
    val status: MessageStatus,
    val statusText: MessageStatusText,
    val nextAttempt: Instant? = null,
    /** An example endpoint name. */
    val description: String,
    /**
     * Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val url: String,
    val disabled: Boolean? = null,
    val eventTypes: Set<String>? = null,
    /** List of message channels this endpoint listens to (omit for all). */
    val channels: Set<String>? = null,
    val createdAt: Instant,
    val updatedAt: Instant,
)
