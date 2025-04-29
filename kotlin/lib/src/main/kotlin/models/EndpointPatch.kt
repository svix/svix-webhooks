// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class EndpointPatch(
    val channels: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val metadata: Map<String, String>? = null,
    val rateLimit: MaybeUnset<UShort> = MaybeUnset.Unset,
    /**
     * The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended to
     * not set this and let the server generate the secret.
     */
    val secret: MaybeUnset<String> = MaybeUnset.Unset,
    /** The Endpoint's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
    val url: String? = null,
    val version: UShort? = null,
)
