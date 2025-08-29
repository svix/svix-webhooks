// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class IngestEndpointTransformationPatch(
    val code: MaybeUnset<String> = MaybeUnset.Unset,
    val enabled: Boolean? = null,
)
