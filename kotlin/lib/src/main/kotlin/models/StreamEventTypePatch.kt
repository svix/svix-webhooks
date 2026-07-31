// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class StreamEventTypePatch(
    val description: MaybeUnset<String> = MaybeUnset.Unset,
    val featureFlags: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val deprecated: Boolean? = null,
    val archived: Boolean? = null,
)
