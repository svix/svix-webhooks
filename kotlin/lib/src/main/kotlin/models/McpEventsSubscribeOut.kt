// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class McpEventsSubscribeOut(
    /** The subscription's ID, which is also the UID of its endpoint. */
    val id: String,
    /** The Endpoint's ID. */
    val endpointId: String,
    /** When the subscription expires, or `null` if it never does. */
    val refreshBefore: Instant? = null,
    /** Always `null`, since events can't be replayed. */
    val cursor: String? = null,
    /** Always `false`, since events can't be replayed. */
    val truncated: Boolean,
)
