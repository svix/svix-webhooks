// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ApplicationIn(
    val metadata: Map<String, String>? = null,
    /** Application name for human consumption. */
    val name: String,
    /** Deprecated, use `throttleRate` instead. */
    val rateLimit: UShort? = null,
    /**
     * Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** Optional unique identifier for the application. */
    val uid: String? = null,
)
