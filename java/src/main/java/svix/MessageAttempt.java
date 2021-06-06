package svix;

import svix.ApiException;
import svix.openapi.MessageAttemptApi;
import svix.openapi.model.ListResponseEndpointMessageOut;
import svix.openapi.model.ListResponseMessageAttemptEndpointOut;
import svix.openapi.model.ListResponseMessageAttemptOut;
import svix.openapi.model.ListResponseMessageEndpointOut;
import svix.openapi.model.MessageAttemptOut;
import svix.openapi.model.MessageStatus;

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
