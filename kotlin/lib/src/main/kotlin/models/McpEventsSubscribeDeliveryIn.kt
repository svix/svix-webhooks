// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class McpEventsSubscribeDeliveryIn(
    /** How events are delivered. Only `webhook` is supported. */
    val mode: McpEventsDeliveryMode,
    /** The callback URL events are sent to. */
    val url: String,
    /** The secret events are signed with. MCP Events requires a `whsec_` secret. */
    val secret: String,
)
