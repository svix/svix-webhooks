// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SinkSecretOut(
    /**
     * The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended to
     * not set this and let the server generate the secret.
     */
    val key: String? = null
)
