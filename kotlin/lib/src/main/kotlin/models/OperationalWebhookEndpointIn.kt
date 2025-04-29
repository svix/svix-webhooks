// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class OperationalWebhookEndpointIn(
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val metadata: Map<String, String>? = null,
    val rateLimit: UShort? = null,
    /**
     * The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended to
     * not set this and let the server generate the secret.
     */
    val secret: String? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val url: String,
)
