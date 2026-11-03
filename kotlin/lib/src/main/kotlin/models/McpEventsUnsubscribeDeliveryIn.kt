// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class McpEventsUnsubscribeDeliveryIn(
    /** How events are delivered. Only `webhook` is supported. */
    val mode: McpEventsDeliveryMode,
    /** The callback URL the subscription was made with. */
    val url: String,
)
