package com.svix;

import com.svix.api.MessageApi;
import com.svix.models.ListResponseMessageOut;
import com.svix.models.MessageIn;
import com.svix.models.MessageOut;

public final class Message {
	private final MessageApi api;

	Message() {
		api = new MessageApi();
	}

	public ListResponseMessageOut list(final String appId, final FetchOptions options) throws ApiException {
		return api.listMessagesApiV1AppAppIdMsgGet(appId, options.getIterator(), options.getLimit());
	}

	public MessageOut create(final String appId, final MessageIn messageIn) throws ApiException {
		return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn);
	}

	public MessageOut get(final String msgId, final String appId) throws ApiException {
		return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId);
	}
}
