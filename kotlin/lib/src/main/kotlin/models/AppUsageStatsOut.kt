// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AppUsageStatsOut(
    /**
     * Any app IDs or UIDs received in the request that weren't found.
     *
     * Stats will be produced for all the others.
     */
    val unresolvedAppIds: Set<String>,
    /** The QueueBackgroundTask's ID. */
    val id: String,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
    val updatedAt: Instant,
)
