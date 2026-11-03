// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class McpEventsUnsubscribeIn(
    /** The event the subscription was made for. */
    val name: String,
    @Serializable(with = StringAnyMapSerializer::class)
    /**
     * The filter arguments the subscription was made with, as given to `events/subscribe`. See
     * [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
     * in OpenAI's MCP Events docs.
     */
    val arguments: Map<String, Any>? = null,
    val delivery: McpEventsUnsubscribeDeliveryIn,
)
