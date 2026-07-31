// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ReplayOut(
    /** The QueueBackgroundTask's ID. */
    val id: String,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
    val updatedAt: Instant,
)
