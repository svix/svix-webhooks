package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.MessageApi;
import com.svix.generated.model.ListResponseMessageOut;
import com.svix.generated.model.MessageIn;
import com.svix.generated.model.MessageOut;

public class Message {
	private final MessageApi api;

	Message() {
		api = new MessageApi();
	}

	public ListResponseMessageOut list(String appId, String iterator, Integer limit) throws ApiException {
		return api.listMessagesApiV1AppAppIdMsgGet(appId, iterator, limit);
	}

	public MessageOut create(String appId, MessageIn messageIn) throws ApiException {
		return api.createMessageApiV1AppAppIdMsgPost(appId, messageIn);
	}

	public MessageOut get(String msgId, String appId) throws ApiException {
		return api.getMessageApiV1AppAppIdMsgMsgIdGet(msgId, appId);
	}
}
