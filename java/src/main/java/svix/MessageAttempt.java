package svix;

import openapi.svix.ApiException;
import openapi.svix.api.MessageAttemptApi;
import openapi.svix.model.ListResponseEndpointMessageOut;
import openapi.svix.model.ListResponseMessageAttemptEndpointOut;
import openapi.svix.model.ListResponseMessageAttemptOut;
import openapi.svix.model.ListResponseMessageEndpointOut;
import openapi.svix.model.MessageAttemptOut;
import openapi.svix.model.MessageStatus;

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
