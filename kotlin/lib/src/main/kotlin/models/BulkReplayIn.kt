// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class BulkReplayIn(
    val since: Instant,
    val until: Instant? = null,
    val eventTypes: Set<String>? = null,
    val channel: String? = null,
    val tag: String? = null,
    val status: MessageStatus? = null,
    val statusCodeClass: StatusCodeClass? = null,
)
