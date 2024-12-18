package com.svix;

import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.MessageApi;
import com.svix.models.ListResponseMessageOut;
import com.svix.models.MessageIn;
import com.svix.models.MessageOut;
import com.google.gson.JsonParser;

public final class Message {
	private final MessageApi api;

	Message() {
		api = new MessageApi();
	}

	private static MessageIn messageInEmptyPayload() {
		return new MessageIn()
			.payload(Collections.emptyMap());
	}

	/**
	 * Creates a MessageIn with a raw string payload.
	 *
	 * The payload is not normalized on the server. Normally, payloads are
	 * required to be JSON, and Svix will minify the payload before sending the
	 * webhooks (for example, by removing extraneous whitespace or unnecessarily
	 * escaped characters in strings). With this function, the payload will be
	 * sent "as is", without any minification or other processing.
	 *
	 * @param payload Serialized message payload
	 */
	public static MessageIn messageInRaw(final String payload) {
		return messageInEmptyPayload().transformationsParams(
			Collections.singletonMap("rawPayload", payload)
		);
	}

	/**
	 * Creates a MessageIn with a raw string payload.
	 *
	 * This overload is intended for non-JSON payloads.
	 *
	 * @param payload Serialized message payload
	 * @param contentType The value to use for the Content-Type header of the webhook sent by Svix
	 */
	public static MessageIn messageInRaw(final String payload, final String contentType) {
		HashMap<String, Object> trParam = new HashMap<>();
		trParam.put("rawPayload", payload);
		trParam.put("headers", Collections.singletonMap("content-type", contentType));
		return messageInEmptyPayload().transformationsParams(trParam);
	}

	public ListResponseMessageOut list(final String appId, final MessageListOptions options) throws ApiException {
		try {
			return api.v1MessageList(
				appId,
				options.getLimit(),
				options.getIterator(),
				options.getChannel(),
				options.getBefore(),
				options.getAfter(),
				options.getWithContent(),
				options.getTag(),
				new HashSet<>(options.getEventTypes())
			);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public MessageOut create(final String appId, final MessageIn messageIn) throws ApiException {
		return this.create(appId, messageIn, new PostOptions());
	}

	public MessageOut create(final String appId, final MessageIn messageIn, final PostOptions options) throws ApiException {
		try {
			return api.v1MessageCreate(appId, messageIn.payload(getPayload(messageIn.getPayload())), null, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public MessageOut get(final String msgId, final String appId) throws ApiException {
		try {
			return api.v1MessageGet(appId, msgId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void expungeContent(final String msgId, final String appId) throws ApiException {
		try {
			api.v1MessageExpungeContent(appId, msgId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	private static Object getPayload(final Object payload) {
		// Convert string to JsonObject, otherwise gson fails to convert it.
		if (payload instanceof String) {
			return new JsonParser().parse(payload.toString());
		}
		return payload;
	}
}
