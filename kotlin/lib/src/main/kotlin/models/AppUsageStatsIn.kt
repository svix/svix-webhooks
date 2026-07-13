// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AppUsageStatsIn(
    val since: Instant,
    val until: Instant,
    /**
     * Specific app IDs or UIDs to aggregate stats for.
     *
     * Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
     */
    val appIds: Set<String>? = null,
)
