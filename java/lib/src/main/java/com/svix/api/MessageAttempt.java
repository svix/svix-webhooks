// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.EmptyResponse;
import com.svix.models.ListResponseEndpointMessageOut;
import com.svix.models.ListResponseMessageAttemptOut;
import com.svix.models.ListResponseMessageEndpointOut;
import com.svix.models.MessageAttemptOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class MessageAttempt {
    private final SvixHttpClient client;

    public MessageAttempt(SvixHttpClient client) {
        this.client = client;
    }

    /**
     * List attempts by endpoint id
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageAttemptOut listByEndpoint(final String appId, final String endpointId)
            throws IOException, ApiException {
        return this.listByEndpoint(appId, endpointId, new MessageAttemptListByEndpointOptions());
    }

    /**
     * List attempts by endpoint id
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageAttemptOut listByEndpoint(
            final String appId,
            final String endpointId,
            final MessageAttemptListByEndpointOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/attempt/endpoint/%s", appId, endpointId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.status != null) {
            url.addQueryParameter("status", Utils.serializeQueryParam(options.status));
        }
        if (options.statusCodeClass != null) {
            url.addQueryParameter(
                    "status_code_class", Utils.serializeQueryParam(options.statusCodeClass));
        }
        if (options.channel != null) {
            url.addQueryParameter("channel", options.channel);
        }
        if (options.tag != null) {
            url.addQueryParameter("tag", options.tag);
        }
        if (options.before != null) {
            url.addQueryParameter("before", Utils.serializeQueryParam(options.before));
        }
        if (options.after != null) {
            url.addQueryParameter("after", Utils.serializeQueryParam(options.after));
        }
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        if (options.withMsg != null) {
            url.addQueryParameter("with_msg", Utils.serializeQueryParam(options.withMsg));
        }
        if (options.eventTypes != null) {
            url.addQueryParameter("event_types", Utils.serializeQueryParam(options.eventTypes));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseMessageAttemptOut.class);
    }

    /**
     * List attempts by message ID.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageAttemptOut listByMsg(final String appId, final String msgId)
            throws IOException, ApiException {
        return this.listByMsg(appId, msgId, new MessageAttemptListByMsgOptions());
    }

    /**
     * List attempts by message ID.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageAttemptOut listByMsg(
            final String appId, final String msgId, final MessageAttemptListByMsgOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/attempt/msg/%s", appId, msgId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.status != null) {
            url.addQueryParameter("status", Utils.serializeQueryParam(options.status));
        }
        if (options.statusCodeClass != null) {
            url.addQueryParameter(
                    "status_code_class", Utils.serializeQueryParam(options.statusCodeClass));
        }
        if (options.channel != null) {
            url.addQueryParameter("channel", options.channel);
        }
        if (options.tag != null) {
            url.addQueryParameter("tag", options.tag);
        }
        if (options.endpointId != null) {
            url.addQueryParameter("endpoint_id", options.endpointId);
        }
        if (options.before != null) {
            url.addQueryParameter("before", Utils.serializeQueryParam(options.before));
        }
        if (options.after != null) {
            url.addQueryParameter("after", Utils.serializeQueryParam(options.after));
        }
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        if (options.eventTypes != null) {
            url.addQueryParameter("event_types", Utils.serializeQueryParam(options.eventTypes));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseMessageAttemptOut.class);
    }

    /**
     * List messages for a particular endpoint. Additionally includes metadata about the latest
     * message attempt.
     *
     * <p>The `before` parameter lets you filter all items created before a certain date and is
     * ignored if an iterator is passed.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseEndpointMessageOut listAttemptedMessages(
            final String appId, final String endpointId) throws IOException, ApiException {
        return this.listAttemptedMessages(
                appId, endpointId, new MessageAttemptListAttemptedMessagesOptions());
    }

    /**
     * List messages for a particular endpoint. Additionally includes metadata about the latest
     * message attempt.
     *
     * <p>The `before` parameter lets you filter all items created before a certain date and is
     * ignored if an iterator is passed.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseEndpointMessageOut listAttemptedMessages(
            final String appId,
            final String endpointId,
            final MessageAttemptListAttemptedMessagesOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/endpoint/%s/msg", appId, endpointId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.channel != null) {
            url.addQueryParameter("channel", options.channel);
        }
        if (options.tag != null) {
            url.addQueryParameter("tag", options.tag);
        }
        if (options.status != null) {
            url.addQueryParameter("status", Utils.serializeQueryParam(options.status));
        }
        if (options.before != null) {
            url.addQueryParameter("before", Utils.serializeQueryParam(options.before));
        }
        if (options.after != null) {
            url.addQueryParameter("after", Utils.serializeQueryParam(options.after));
        }
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        if (options.eventTypes != null) {
            url.addQueryParameter("event_types", Utils.serializeQueryParam(options.eventTypes));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseEndpointMessageOut.class);
    }

    /** `msg_id`: Use a message id or a message `eventId` */
    public MessageAttemptOut get(final String appId, final String msgId, final String attemptId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/msg/%s/attempt/%s",
                                        appId, msgId, attemptId));
        return this.client.executeRequest("GET", url.build(), null, null, MessageAttemptOut.class);
    }

    /**
     * Deletes the given attempt's response body.
     *
     * <p>Useful when an endpoint accidentally returned sensitive content. The message can't be
     * replayed or resent once its payload has been deleted or expired.
     */
    public void expungeContent(final String appId, final String msgId, final String attemptId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/msg/%s/attempt/%s/content",
                                        appId, msgId, attemptId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /**
     * List endpoints attempted by a given message.
     *
     * <p>Additionally includes metadata about the latest message attempt. By default, endpoints are
     * listed in ascending order by ID.
     */
    public ListResponseMessageEndpointOut listAttemptedDestinations(
            final String appId, final String msgId) throws IOException, ApiException {
        return this.listAttemptedDestinations(
                appId, msgId, new MessageAttemptListAttemptedDestinationsOptions());
    }

    /**
     * List endpoints attempted by a given message.
     *
     * <p>Additionally includes metadata about the latest message attempt. By default, endpoints are
     * listed in ascending order by ID.
     */
    public ListResponseMessageEndpointOut listAttemptedDestinations(
            final String appId,
            final String msgId,
            final MessageAttemptListAttemptedDestinationsOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/msg/%s/endpoint", appId, msgId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseMessageEndpointOut.class);
    }

    /** Resend a message to the specified endpoint. */
    public EmptyResponse resend(final String appId, final String msgId, final String endpointId)
            throws IOException, ApiException {
        return this.resend(appId, msgId, endpointId, new MessageAttemptResendOptions());
    }

    /** Resend a message to the specified endpoint. */
    public EmptyResponse resend(
            final String appId,
            final String msgId,
            final String endpointId,
            final MessageAttemptResendOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/msg/%s/endpoint/%s/resend",
                                        appId, msgId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), null, EmptyResponse.class);
    }
}
