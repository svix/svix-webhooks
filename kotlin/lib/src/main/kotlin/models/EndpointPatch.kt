// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class EndpointPatch(
    val description: String? = null,
    /**
     * Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: MaybeUnset<UShort> = MaybeUnset.Unset,
    /** The Endpoint's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
    val url: String? = null,
    val disabled: Boolean? = null,
    val eventTypes: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val channels: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val metadata: Map<String, String>? = null,
)
