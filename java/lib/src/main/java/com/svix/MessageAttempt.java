package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.MessageAttemptApi;
import com.svix.models.ListResponseEndpointMessageOut;
import com.svix.models.ListResponseMessageAttemptEndpointOut;
import com.svix.models.ListResponseMessageAttemptOut;
import com.svix.models.ListResponseMessageEndpointOut;
import com.svix.models.MessageAttemptOut;

public final class MessageAttempt {
	private final MessageAttemptApi api;

	MessageAttempt() {
		api = new MessageAttemptApi();
	}

	public ListResponseMessageAttemptOut list(final String appId, final String msgId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(appId, msgId, options.getIterator(), options.getLimit(), options.getMessageStatus());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public MessageAttemptOut get(final String msgId, final String appId, final String attemptId) throws ApiException {
		try {
			return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void resend(final String msgId, final String appId, final String endpointId) throws ApiException {
		try {
			api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseEndpointMessageOut listAttemptedMessages(final String appId, final String endpointId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(endpointId, appId, options.getIterator(), options.getLimit(), options.getMessageStatus(), options.getBefore());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseMessageEndpointOut listAttemptedDestinations(final String appId, final String msgId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, options.getIterator(), options.getLimit());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseMessageAttemptEndpointOut listAttemptsForEndpoint(final String appId, final String msgId, final String endpointId,
		final MessageAttemptListOptions options) throws ApiException {
			try {
				return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(msgId, appId, endpointId,
					options.getIterator(), options.getLimit(), options.getEventTypes(), options.getMessageStatus(), options.getBefore());
			} catch (com.svix.internal.ApiException e) {
				throw Utils.wrapInternalApiException(e);
			}
	}
}
