// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.McpEventsSubscribeIn
import com.svix.kotlin.models.McpEventsSubscribeOut
import com.svix.kotlin.models.McpEventsUnsubscribeIn
import okhttp3.Headers

data class EndpointMcpEventsSubscribeOptions(val idempotencyKey: String? = null)

data class EndpointMcpEventsUnsubscribeOptions(val idempotencyKey: String? = null)

class EndpointMcpEvents(private val client: SvixHttpClient) {
    /** Create or refresh an MCP Events subscription. */
    suspend fun subscribe(
        appId: String,
        mcpEventsSubscribeIn: McpEventsSubscribeIn,
        options: EndpointMcpEventsSubscribeOptions = EndpointMcpEventsSubscribeOptions(),
    ): McpEventsSubscribeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/mcp-events/subscribe")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<McpEventsSubscribeIn, McpEventsSubscribeOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = mcpEventsSubscribeIn,
        )
    }

    /**
     * Stop an MCP Events subscription.
     *
     * Forward an MCP server's `events/unsubscribe` requests here. The subscription's endpoint is
     * deleted. Unsubscribing from a subscription that doesn't exist succeeds, so this can be
     * repeated.
     */
    suspend fun unsubscribe(
        appId: String,
        mcpEventsUnsubscribeIn: McpEventsUnsubscribeIn,
        options: EndpointMcpEventsUnsubscribeOptions = EndpointMcpEventsUnsubscribeOptions(),
    ) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/mcp-events/unsubscribe")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<McpEventsUnsubscribeIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = mcpEventsUnsubscribeIn,
        )
    }
}
