// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ApplicationIn(
    /** Application name for human consumption. */
    val name: String,
    /**
     * Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: UShort? = null,
    /** Optional unique identifier for the application. */
    val uid: String? = null,
    val metadata: Map<String, String>? = null,
)
