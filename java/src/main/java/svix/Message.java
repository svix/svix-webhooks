package svix;

import openapi.svix.ApiException;
import openapi.svix.api.MessageApi;
import openapi.svix.model.ListResponseMessageOut;
import openapi.svix.model.MessageIn;
import openapi.svix.model.MessageOut;

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
