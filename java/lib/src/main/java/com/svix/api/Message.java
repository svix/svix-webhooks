// this file is @generated
package com.svix.api;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.*;
import com.svix.models.ExpungeAllContentsOut;
import com.svix.models.ListResponseMessageOut;
import com.svix.models.MessageIn;
import com.svix.models.MessageOut;

import lombok.*;
import lombok.Getter;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class Message {
    private final SvixHttpClient client;

    @Getter private final MessagePoller poller;

    public Message(SvixHttpClient client) {
        this.client = client;
        this.poller = new MessagePoller(client);
    }

    /**
     * List all of the application's messages.
     *
     * <p>The `before` and `after` parameters let you filter all items created before or after a
     * certain date. These can be used alongside an iterator to paginate over results within a
     * certain window.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageOut list(final String appId) throws IOException, ApiException {
        return this.list(appId, new MessageListOptions());
    }

    /**
     * List all of the application's messages.
     *
     * <p>The `before` and `after` parameters let you filter all items created before or after a
     * certain date. These can be used alongside an iterator to paginate over results within a
     * certain window.
     *
     * <p>Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated by
     * the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public ListResponseMessageOut list(final String appId, final MessageListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s/msg", appId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.channel != null) {
            url.addQueryParameter("channel", options.channel);
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
        if (options.tag != null) {
            url.addQueryParameter("tag", options.tag);
        }
        if (options.eventTypes != null) {
            url.addQueryParameter("event_types", Utils.serializeQueryParam(options.eventTypes));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseMessageOut.class);
    }

    /**
     * Creates a new message and dispatches it to all of the application's endpoints.
     *
     * <p>The `eventId` is an optional custom unique ID. It's verified to be unique only up to a
     * day, after that no verification will be made. If a message with the same `eventId` already
     * exists for the application, a 409 conflict error will be returned.
     *
     * <p>The `eventType` indicates the type and schema of the event. All messages of a certain
     * `eventType` are expected to have the same schema. Endpoints can choose to only listen to
     * specific event types. Messages can also have `channels`, which similar to event types let
     * endpoints filter by them. Unlike event types, messages can have multiple channels, and
     * channels don't imply a specific message content or schema.
     *
     * <p>The `payload` property is the webhook's body (the actual webhook message). Svix supports
     * payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads
     * small, probably no larger than 40kb.
     */
    public MessageOut create(final String appId, final MessageIn messageIn)
            throws IOException, ApiException {
        return this.create(appId, messageIn, new MessageCreateOptions());
    }

    /**
     * Creates a new message and dispatches it to all of the application's endpoints.
     *
     * <p>The `eventId` is an optional custom unique ID. It's verified to be unique only up to a
     * day, after that no verification will be made. If a message with the same `eventId` already
     * exists for the application, a 409 conflict error will be returned.
     *
     * <p>The `eventType` indicates the type and schema of the event. All messages of a certain
     * `eventType` are expected to have the same schema. Endpoints can choose to only listen to
     * specific event types. Messages can also have `channels`, which similar to event types let
     * endpoints filter by them. Unlike event types, messages can have multiple channels, and
     * channels don't imply a specific message content or schema.
     *
     * <p>The `payload` property is the webhook's body (the actual webhook message). Svix supports
     * payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads
     * small, probably no larger than 40kb.
     */
    public MessageOut create(
            final String appId, final MessageIn messageIn, final MessageCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s/msg", appId));
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        MessageInInternal msgInInternal = new MessageInInternal(messageIn);
        if (msgInInternal.transformationsParams != null) {
            if (msgInInternal.transformationsParams.get("rawPayload") == null) {
                // transformationsParams may be immutable
                HashMap<String, Object> trParams =
                        new HashMap<>(msgInInternal.transformationsParams);
                trParams.put("rawPayload", msgInInternal.payload);
                msgInInternal.transformationsParams = trParams;
            }
        } else {
            HashMap<String, Object> trParam = new HashMap<>();
            trParam.put("rawPayload", msgInInternal.payload);
            msgInInternal.transformationsParams = trParam;
        }
        msgInInternal.payload = new HashMap<>();
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), msgInInternal, MessageOut.class);
    }

    /**
     * Delete all message payloads for the application.
     *
     * <p>This operation is only available in the <a href="https://svix.com/pricing"
     * target="_blank">Enterprise</a> plan.
     */
    public ExpungeAllContentsOut expungeAllContents(final String appId)
            throws IOException, ApiException {
        return this.expungeAllContents(appId, new MessageExpungeAllContentsOptions());
    }

    /**
     * Delete all message payloads for the application.
     *
     * <p>This operation is only available in the <a href="https://svix.com/pricing"
     * target="_blank">Enterprise</a> plan.
     */
    public ExpungeAllContentsOut expungeAllContents(
            final String appId, final MessageExpungeAllContentsOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/msg/expunge-all-contents", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), null, ExpungeAllContentsOut.class);
    }

    /** Get a message by its ID or eventID. */
    public MessageOut get(final String appId, final String msgId) throws IOException, ApiException {
        return this.get(appId, msgId, new MessageGetOptions());
    }

    /** Get a message by its ID or eventID. */
    public MessageOut get(final String appId, final String msgId, final MessageGetOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/msg/%s", appId, msgId));
        if (options.withContent != null) {
            url.addQueryParameter("with_content", Utils.serializeQueryParam(options.withContent));
        }
        return this.client.executeRequest("GET", url.build(), null, null, MessageOut.class);
    }

    /**
     * Delete the given message's payload.
     *
     * <p>Useful in cases when a message was accidentally sent with sensitive content. The message
     * can't be replayed or resent once its payload has been deleted or expired.
     */
    public void expungeContent(final String appId, final String msgId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/msg/%s/content", appId, msgId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /**
     * Creates a MessageIn with a raw string payload.
     *
     * <p>The payload is not normalized on the server. Normally, payloads are required to be JSON,
     * and Svix will minify the payload before sending the webhooks (for example, by removing
     * extraneous whitespace or unnecessarily escaped characters in strings). With this function,
     * the payload will be sent "as is", without any minification or other processing.
     *
     * @param payload Serialized message payload
     */
    public static MessageIn messageInRaw(final String payload) {
        MessageIn msg = new MessageIn();
        msg.setPayload("");
        msg.setTransformationsParams(Collections.singletonMap("rawPayload", payload));
        return msg;
    }

    /**
     * Creates a MessageIn with a raw string payload.
     *
     * <p>This overload is intended for non-JSON payloads.
     *
     * @param payload Serialized message payload
     * @param contentType The value to use for the Content-Type header of the webhook sent by Svix
     */
    public static MessageIn messageInRaw(final String payload, final String contentType) {
        HashMap<String, Object> trParam = new HashMap<>();
        trParam.put("rawPayload", payload);
        trParam.put("headers", Collections.singletonMap("content-type", contentType));
        MessageIn msg = new MessageIn();
        msg.setPayload("");
        msg.setTransformationsParams(trParam);
        return msg;
    }

    @ToString
    @EqualsAndHashCode
    @JsonInclude(JsonInclude.Include.NON_NULL)
    @NoArgsConstructor
    // we use this because we need payload to be an object while the public `MessageIn.payload` is a
    // string
    private class MessageInInternal {
        @JsonProperty private ApplicationIn application;
        @JsonProperty private Set<String> channels;
        @JsonProperty private String eventId;
        @JsonProperty private String eventType;
        @JsonProperty private Object payload;
        @JsonProperty private Long payloadRetentionHours;
        @JsonProperty private Long payloadRetentionPeriod;
        @JsonProperty private Set<String> tags;
        @JsonProperty private Map<String, Object> transformationsParams;

        private MessageInInternal(MessageIn messageIn) {
            this.application = messageIn.getApplication();
            this.channels = messageIn.getChannels();
            this.eventId = messageIn.getEventId();
            this.eventType = messageIn.getEventType();
            this.payload = messageIn.getPayload();
            this.payloadRetentionHours = messageIn.getPayloadRetentionHours();
            this.payloadRetentionPeriod = messageIn.getPayloadRetentionPeriod();
            this.tags = messageIn.getTags();
            this.transformationsParams = messageIn.getTransformationsParams();
        }
    }
}
