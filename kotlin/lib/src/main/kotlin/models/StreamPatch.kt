// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class StreamPatch(
    /** The Stream's description. */
    val description: String? = null,
    val metadata: Map<String, String>? = null,
    /** An optional unique identifier for the stream. */
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
)
