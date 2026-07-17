// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointUpsertIn(
    val url: String,
    val description: String? = null,
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
    /** List of message channels this endpoint listens to (omit for all). */
    val channels: Set<String>? = null,
    val metadata: Map<String, String>? = null,
)
