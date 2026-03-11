// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointUpdate(
    /** List of message channels this endpoint listens to (omit for all). */
    val channels: Set<String>? = null,
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val metadata: Map<String, String>? = null,
    /** Deprecated, use `throttleRate` instead. */
    val rateLimit: UShort? = null,
    /**
     * Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val url: String,
    val version: UShort? = null,
)
