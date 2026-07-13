// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EndpointTransformationOut(
    val code: String? = null,
    val enabled: Boolean? = null,
    val variables: Map<String, String>? = null,
    val updatedAt: Instant? = null,
)
