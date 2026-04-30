// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class ApplicationPatch(
    val metadata: Map<String, String>? = null,
    val name: String? = null,
    /** Deprecated, use `throttleRate` instead. */
    val rateLimit: MaybeUnset<UShort> = MaybeUnset.Unset,
    /**
     * Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     */
    val throttleRate: MaybeUnset<UShort> = MaybeUnset.Unset,
    /** The Application's UID. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
)
