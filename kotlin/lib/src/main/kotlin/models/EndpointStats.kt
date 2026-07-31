// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointStats(
    val success: Long,
    val pending: Long,
    val sending: Long,
    val fail: Long,
    val canceled: Long,
)
