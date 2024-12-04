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
	 * Creates a MessageIn with the payload already being serialized.
	 *
	 * The payload is not normalized on the server (usually whitespace outside
	 * of string literals, unnecessarily escaped characters in string and such
	 * are fixed up by the server). The default Content-Type of application/json
	 * is used. See the other overload if you want to send non-JSON payloads.
	 *
	 * @param payload Serialized message payload
	 */
	public static MessageIn messageInRaw(final String payload) {
		return messageInEmptyPayload().transformationsParams(
			Collections.singletonMap("rawPayload", payload)
		);
	}

	/**
	 * Creates a MessageIn with the payload already being serialized.
	 *
	 * This overload is intended for non-JSON payloads.
	 *
	 * @param payload Serialized message payload
	 * @param contentType The value to use for the Content-Type header
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
