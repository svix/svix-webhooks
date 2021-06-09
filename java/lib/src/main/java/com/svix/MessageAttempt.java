package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.MessageAttemptApi;
import com.svix.generated.model.ListResponseEndpointMessageOut;
import com.svix.generated.model.ListResponseMessageAttemptEndpointOut;
import com.svix.generated.model.ListResponseMessageAttemptOut;
import com.svix.generated.model.ListResponseMessageEndpointOut;
import com.svix.generated.model.MessageAttemptOut;

public final class MessageAttempt {
	private final MessageAttemptApi api;

	MessageAttempt() {
		api = new MessageAttemptApi();
	}

	public ListResponseMessageAttemptOut list(final String appId, final String msgId, final FetchOptionsMessageAttempt options) throws ApiException {
		return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(msgId, appId, options.getIterator(), options.getLimit(), options.getMessageStatus());
	}

	public MessageAttemptOut get(final String msgId, final String appId, final String attemptId) throws ApiException {
		return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId);
	}

	public Object resend(final String msgId, final String appId, final String endpointId) throws ApiException {
		return api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId);
	}

	public ListResponseEndpointMessageOut listAttemptedMessages(final String appId, final String endpointId, final FetchOptionsMessageAttempt options) throws ApiException {
		return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, options.getIterator(), options.getLimit(), options.getMessageStatus());
	}

	public ListResponseMessageEndpointOut listAttemptedDestinations(final String appId, final String msgId, final FetchOptions options) throws ApiException {
		return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, options.getIterator(), options.getLimit());
	}

	public ListResponseMessageAttemptEndpointOut listAttemptsForEndpoint(final String appId, final String msgId, final String endpointId,
		final FetchOptionsMessageAttempt options) throws ApiException {
		return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId,
			options.getIterator(), options.getLimit(), options.getMessageStatus());
	}
}
