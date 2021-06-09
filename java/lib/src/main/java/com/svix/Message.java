package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.MessageApi;
import com.svix.model.ListResponseMessageOut;
import com.svix.model.MessageIn;
import com.svix.model.MessageOut;

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
