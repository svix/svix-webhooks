// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AppUsageStatsIn(
    /**
     * Specific app IDs or UIDs to aggregate stats for.
     *
     * Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
     */
    val appIds: Set<String>? = null,
    val since: Instant,
    val until: Instant,
)
