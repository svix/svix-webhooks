// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class TailscaleConfig(
    /** Shared secret for Tailscale Webhooks */
    val secret: String,
    /**
     * Grace period (in seconds) for the timestamp.
     *
     * If not passed, timestamp age will not be checked.
     */
    val timestampGraceSeconds: UInt? = null,
)
