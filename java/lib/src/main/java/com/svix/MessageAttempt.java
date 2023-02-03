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

	/*
	* @deprecated: use listByMsg or listByEndpoint instead
     */
	@Deprecated
	public ListResponseMessageAttemptOut list(final String appId, final String msgId, final MessageAttemptListOptions options) throws ApiException {
		return this.listByMsg(appId, msgId, options);
	}

	public ListResponseMessageAttemptOut listByMsg(final String appId, final String msgId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptsByMsgApiV1AppAppIdAttemptMsgMsgIdGet(
					appId,
					msgId,
					null,
					options.getIterator(),
					options.getLimit(),
					options.getMessageStatus(),
					options.getStatusCodeClass(),
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

	public ListResponseMessageAttemptOut listByEndpoint(final String appId, final String endpointId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptsByEndpointApiV1AppAppIdAttemptEndpointEndpointIdGet(
					appId,
					endpointId,
					options.getIterator(),
					options.getLimit(),
					options.getMessageStatus(),
					options.getStatusCodeClass(),
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

	public MessageAttemptOut get(final String msgId, final String appId, final String attemptId) throws ApiException {
		try {
			return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void resend(final String msgId, final String appId, final String endpointId) throws ApiException {
		this.resend(msgId, appId, endpointId, new PostOptions());
	}

	public void resend(final String msgId, final String appId, final String endpointId, final PostOptions options) throws ApiException {
		try {
			api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseEndpointMessageOut listAttemptedMessages(final String appId, final String endpointId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(
					endpointId,
					appId,
					options.getIterator(),
					options.getLimit(),
					options.getChannel(),
					options.getMessageStatus(),
					options.getBefore(),
					options.getAfter(),
					null
			);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseMessageEndpointOut listAttemptedDestinations(final String appId, final String msgId, final MessageAttemptListOptions options) throws ApiException {
		try {
			return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(msgId, appId, options.getIterator(), options.getLimit(), null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ListResponseMessageAttemptEndpointOut listAttemptsForEndpoint(final String appId, final String msgId, final String endpointId,
		final MessageAttemptListOptions options) throws ApiException {
			try {
				return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(
						msgId,
						appId,
						endpointId,
						options.getIterator(),
						options.getLimit(),
						options.getEventTypes(),
						options.getChannel(),
						options.getMessageStatus(),
						options.getBefore(),
						options.getAfter(),
						null
				);
			} catch (com.svix.internal.ApiException e) {
				throw Utils.wrapInternalApiException(e);
			}
	}

	public void expungeContent(final String appId, final String msgId, final String attemptId) throws ApiException {
		try {
			api.expungeAttemptContentApiV1AppAppIdMsgMsgIdAttemptAttemptIdContentDelete(
				attemptId,
				msgId,
				appId,
				null
			);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
