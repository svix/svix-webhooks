// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ApplicationOut(
    /** Optional unique identifier for the application. */
    val uid: String? = null,
    /** Application name for human consumption. */
    val name: String,
    /**
     * Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** The Application's ID. */
    val id: String,
    val createdAt: Instant,
    val updatedAt: Instant,
    val metadata: Map<String, String>,
)
