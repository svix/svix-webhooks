// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointIn(
    /** List of message channels this endpoint listens to (omit for all). */
    val channels: Set<String>? = null,
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val headers: Map<String, String>? = null,
    val metadata: Map<String, String>? = null,
    /** Deprecated, use `throttleRate` instead. */
    val rateLimit: UShort? = null,
    /**
     * The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended to
     * not set this and let the server generate the secret.
     */
    val secret: String? = null,
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
