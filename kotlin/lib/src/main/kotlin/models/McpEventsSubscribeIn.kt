// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class McpEventsSubscribeIn(
    /** The event to subscribe to. It must be an existing event type. */
    val name: String,
    @Serializable(with = StringAnyMapSerializer::class)
    /**
     * The filter arguments of the `events/subscribe` request, as described by the event's
     * `inputSchema` (e.g. `{"document_id": "doc_123"}`). Together with `name` and the callback URL
     * they identify the subscription
     *
     * They're stored in the endpoint's `mcpEventArguments` metadata, but aren't used to filter
     * messages; use `channels` for that.
     *
     * See
     * [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
     * in OpenAI's MCP Events docs.
     */
    val arguments: Map<String, Any>? = null,
    val delivery: McpEventsSubscribeDeliveryIn,
    /**
     * The requested lifetime of the subscription, in milliseconds. Omit for the default of 7 days,
     * or set to `null` for a subscription that never expires.
     */
    val ttlMs: ULong? = null,
    /**
     * How long the previous secret keeps signing when a refresh changes it, in seconds.
     *
     * Valid values are between 0 (immediate expiry) and 7 days. The default is 5 minutes.
     */
    val secretGracePeriodSeconds: UInt? = null,
    /** How the callback must respond to the verification challenge. */
    val challengeMode: EndpointChallengeMode? = null,
    /**
     * Only send messages with one of these channels to the subscription, for example ones derived
     * from `arguments`. Omit to send all messages of the event type.
     */
    val channels: Set<String>? = null,
)
