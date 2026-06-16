// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.PollerV2CommitIn;
import com.svix.models.PollerV2PollOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class MessagePollerv2 {
    private final SvixHttpClient client;

    public MessagePollerv2(SvixHttpClient client) {
        this.client = client;
    }

    /** Poll messages from a sink. */
    public PollerV2PollOut consumerPoll(
            final String appId, final String sinkId, final String consumerId)
            throws IOException, ApiException {
        return this.consumerPoll(
                appId, sinkId, consumerId, new MessagePollerv2ConsumerPollOptions());
    }

    /** Poll messages from a sink. */
    public PollerV2PollOut consumerPoll(
            final String appId,
            final String sinkId,
            final String consumerId,
            final MessagePollerv2ConsumerPollOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/polling-endpoint/%s/consumer/%s",
                                        appId, sinkId, consumerId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.leaseDurationMs != null) {
            url.addQueryParameter(
                    "lease_duration_ms", Utils.serializeQueryParam(options.leaseDurationMs));
        }
        if (options.startingPosition != null) {
            url.addQueryParameter(
                    "starting_position", Utils.serializeQueryParam(options.startingPosition));
        }
        return this.client.executeRequest("GET", url.build(), null, null, PollerV2PollOut.class);
    }

    /** Ack a message offset for a sink's consumer. */
    public void consumerCommit(
            final String appId,
            final String sinkId,
            final String consumerId,
            final PollerV2CommitIn pollerV2CommitIn)
            throws IOException, ApiException {
        this.consumerCommit(
                appId,
                sinkId,
                consumerId,
                pollerV2CommitIn,
                new MessagePollerv2ConsumerCommitOptions());
    }

    /** Ack a message offset for a sink's consumer. */
    public void consumerCommit(
            final String appId,
            final String sinkId,
            final String consumerId,
            final PollerV2CommitIn pollerV2CommitIn,
            final MessagePollerv2ConsumerCommitOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/polling-endpoint/%s/consumer/%s/commit",
                                        appId, sinkId, consumerId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), pollerV2CommitIn, null);
    }
}
