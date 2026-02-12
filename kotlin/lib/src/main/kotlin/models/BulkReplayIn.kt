// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class BulkReplayIn(
    val channel: String? = null,
    val eventTypes: Set<String>? = null,
    val since: Instant,
    val status: MessageStatus? = null,
    val tag: String? = null,
    val until: Instant? = null,
)
