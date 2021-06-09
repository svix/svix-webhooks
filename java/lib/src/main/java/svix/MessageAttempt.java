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

	public ListResponseMessageAttemptOut list(final String appId, final String msgId, final String iterator, final Integer limit,
		final MessageStatus status) throws ApiException {
		return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(msgId, appId, iterator, limit, status);
	}

	public MessageAttemptOut get(final String msgId, final String appId, final String attemptId) throws ApiException {
		return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId);
	}

	public Object resend(final String msgId, final String appId, final String endpointId) throws ApiException {
		return api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId);
	}

	public ListResponseEndpointMessageOut listAttemptedMessages(final String appId, final String endpointId, final String iterator,
		final Integer limit, final MessageStatus status) throws ApiException {
		return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, iterator, limit, status);
	}

	public ListResponseMessageEndpointOut listAttemptedDestinations(final String appId, final String msgId, final String iterator,
		final Integer limit) throws ApiException {
		return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, iterator, limit);
	}

	public ListResponseMessageAttemptEndpointOut listAttemptsForEndpoint(final String appId, final String msgId, final String endpointId,
		final String iterator, final Integer limit, final MessageStatus status) throws ApiException {
		return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId,
		    iterator, limit, status);
	}
}
