package com.svix;

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

	public ListResponseMessageOut list(final String appId, final MessageListOptions options) throws ApiException {
		try {
			return api.listMessagesApiV1AppAppIdMsgGet(
					appId,
					options.getIterator(),
					options.getLimit(),
					options.getEventTypes(),
					options.getChannel(),
					options.getBefore(),
					options.getAfter(),
					null
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
			return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn.payload(getPayload(messageIn.getPayload())), null, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public MessageOut get(final String msgId, final String appId) throws ApiException {
		try {
			return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void expungeContent(final String msgId, final String appId) throws ApiException {
		try {
			api.expungeMessagePayloadApiV1AppAppIdMsgMsgIdContentDelete(msgId, appId, null);
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
