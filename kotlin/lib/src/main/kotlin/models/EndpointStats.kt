// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointStats(
    val canceled: Long,
    val fail: Long,
    val pending: Long,
    val sending: Long,
    val success: Long,
)
