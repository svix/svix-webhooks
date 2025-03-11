// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptFailedData(
    val id: String,
    val responseStatusCode: Short,
    val timestamp: Instant,
)
