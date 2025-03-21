// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.PollingEndpointConsumerSeekIn;
import com.svix.models.PollingEndpointConsumerSeekOut;
import com.svix.models.PollingEndpointOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class EventsPublic {
    private final SvixHttpClient client;

    public EventsPublic(SvixHttpClient client) {
        this.client = client;
    }

    /**
     * Reads the stream of created messages for an application, filtered on the Sink's event types
     * and Channels, using server-managed iterator tracking.
     */
    public PollingEndpointOut consumer(
            final String appId, final String sinkId, final String consumerId)
            throws IOException, ApiException {
        return this.consumer(appId, sinkId, consumerId, new EventsPublicConsumerOptions());
    }

    /**
     * Reads the stream of created messages for an application, filtered on the Sink's event types
     * and Channels, using server-managed iterator tracking.
     */
    public PollingEndpointOut consumer(
            final String appId,
            final String sinkId,
            final String consumerId,
            final EventsPublicConsumerOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/poller/%s/consumer/%s",
                                        appId, sinkId, consumerId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.eventType != null) {
            url.addQueryParameter("event_type", options.eventType);
        }
        if (options.channel != null) {
            url.addQueryParameter("channel", options.channel);
        }
        return this.client.executeRequest("GET", url.build(), null, null, PollingEndpointOut.class);
    }

    /** Sets the starting offset for the consumer of a polling endpoint. */
    public PollingEndpointConsumerSeekOut consumerSeek(
            final String appId,
            final String sinkId,
            final String consumerId,
            final PollingEndpointConsumerSeekIn pollingEndpointConsumerSeekIn)
            throws IOException, ApiException {
        return this.consumerSeek(
                appId,
                sinkId,
                consumerId,
                pollingEndpointConsumerSeekIn,
                new EventsPublicConsumerSeekOptions());
    }

    /** Sets the starting offset for the consumer of a polling endpoint. */
    public PollingEndpointConsumerSeekOut consumerSeek(
            final String appId,
            final String sinkId,
            final String consumerId,
            final PollingEndpointConsumerSeekIn pollingEndpointConsumerSeekIn,
            final EventsPublicConsumerSeekOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/poller/%s/consumer/%s/seek",
                                        appId, sinkId, consumerId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                pollingEndpointConsumerSeekIn,
                PollingEndpointConsumerSeekOut.class);
    }
}
