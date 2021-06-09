package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.MessageAttemptApi;
import com.svix.generated.model.ListResponseEndpointMessageOut;
import com.svix.generated.model.ListResponseMessageAttemptEndpointOut;
import com.svix.generated.model.ListResponseMessageAttemptOut;
import com.svix.generated.model.ListResponseMessageEndpointOut;
import com.svix.generated.model.MessageAttemptOut;
import com.svix.generated.model.MessageStatus;

public class MessageAttempt {
	private final MessageAttemptApi api;

	MessageAttempt() {
		api = new MessageAttemptApi();
	}

	public ListResponseMessageAttemptOut list(String appId, String msgId, String iterator, Integer limit,
	    MessageStatus status) throws ApiException {
		return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(msgId, appId, iterator, limit, status);
	}

	public MessageAttemptOut get(String msgId, String appId, String attemptId) throws ApiException {
		return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId);
	}

	public Object resend(String msgId, String appId, String endpointId) throws ApiException {
		return api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId);
	}

	public ListResponseEndpointMessageOut listAttemptedMessages(String appId, String endpointId, String iterator,
	    Integer limit, MessageStatus status) throws ApiException {
		return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, iterator, limit, status);
	}

	public ListResponseMessageEndpointOut listAttemptedDestinations(String appId, String msgId, String iterator,
	    Integer limit) throws ApiException {
		return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, iterator, limit);
	}

	public ListResponseMessageAttemptEndpointOut listAttemptsForEndpoint(String appId, String msgId, String endpointId,
	    String iterator, Integer limit, MessageStatus status) throws ApiException {
		return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId,
		    iterator, limit, status);
	}
}
