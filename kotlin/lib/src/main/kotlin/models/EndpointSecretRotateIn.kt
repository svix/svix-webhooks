// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointSecretRotateIn(
    /**
     * The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended to
     * not set this and let the server generate the secret.
     */
    val key: String? = null,
    /**
     * How long the old secret will be valid for, in seconds.
     *
     * Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours.
     */
    val gracePeriodSeconds: UInt? = null,
)
