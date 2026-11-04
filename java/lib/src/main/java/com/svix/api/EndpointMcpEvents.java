// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.McpEventsSubscribeIn;
import com.svix.models.McpEventsSubscribeOut;
import com.svix.models.McpEventsUnsubscribeIn;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class EndpointMcpEvents {
    private final SvixHttpClient client;

    public EndpointMcpEvents(SvixHttpClient client) {
        this.client = client;
    }

    /** Create or refresh an MCP Events subscription. */
    public McpEventsSubscribeOut subscribe(
            final String appId, final McpEventsSubscribeIn mcpEventsSubscribeIn)
            throws IOException, ApiException {
        return this.subscribe(appId, mcpEventsSubscribeIn, new EndpointMcpEventsSubscribeOptions());
    }

    /** Create or refresh an MCP Events subscription. */
    public McpEventsSubscribeOut subscribe(
            final String appId,
            final McpEventsSubscribeIn mcpEventsSubscribeIn,
            final EndpointMcpEventsSubscribeOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/mcp-events/subscribe", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                mcpEventsSubscribeIn,
                McpEventsSubscribeOut.class);
    }

    /**
     * Stop an MCP Events subscription.
     *
     * <p>Forward an MCP server's `events/unsubscribe` requests here. The subscription's endpoint is
     * deleted. Unsubscribing from a subscription that doesn't exist succeeds, so this can be
     * repeated.
     */
    public void unsubscribe(final String appId, final McpEventsUnsubscribeIn mcpEventsUnsubscribeIn)
            throws IOException, ApiException {
        this.unsubscribe(appId, mcpEventsUnsubscribeIn, new EndpointMcpEventsUnsubscribeOptions());
    }

    /**
     * Stop an MCP Events subscription.
     *
     * <p>Forward an MCP server's `events/unsubscribe` requests here. The subscription's endpoint is
     * deleted. Unsubscribing from a subscription that doesn't exist succeeds, so this can be
     * repeated.
     */
    public void unsubscribe(
            final String appId,
            final McpEventsUnsubscribeIn mcpEventsUnsubscribeIn,
            final EndpointMcpEventsUnsubscribeOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/mcp-events/unsubscribe", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), mcpEventsUnsubscribeIn, null);
    }
}
