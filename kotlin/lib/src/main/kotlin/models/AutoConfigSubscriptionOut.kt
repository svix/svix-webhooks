// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AutoConfigSubscriptionOut(
    val createdAt: Instant,
    val tokenCensored: String,
    /** The AutoConfigSubscription's ID. */
    val id: String,
    /** The Endpoint's ID. */
    val endpId: String? = null,
    /** The StreamSink's ID. */
    val destId: String? = null,
    val status: Status,
)
