package com.svix;

import com.svix.generated.ApiClient;
import com.svix.generated.Configuration;
import com.svix.generated.auth.HttpBearerAuth;

public final class Svix {
	private final Application application;
	private final Authentication authentication;
	private final Endpoint endpoint;
	private final EventType eventType;
	private final Message message;
	private final MessageAttempt messageAttempt;

	public Svix(final String token) {
		this(token, new SvixOptions());
	}

	public Svix(final String token, final SvixOptions options) {
		ApiClient apiClient = Configuration.getDefaultApiClient();
		apiClient.setBasePath(options.getUrl());

		HttpBearerAuth httpBearer = (HttpBearerAuth) apiClient.getAuthentication("HTTPBearer");
		httpBearer.setBearerToken(token);

		Configuration.setDefaultApiClient(apiClient);

		application = new Application();
		authentication = new Authentication();
		endpoint = new Endpoint();
		eventType = new EventType();
		message = new Message();
		messageAttempt = new MessageAttempt();
	}

	public Application getApplication() {
		return application;
	}

	public Authentication getAuthentication() {
		return authentication;
	}

	public Endpoint getEndpoint() {
		return endpoint;
	}

	public EventType getEventType() {
		return eventType;
	}

	public Message getMessage() {
		return message;
	}

	public MessageAttempt getMessageAttempt() {
		return messageAttempt;
	}
}
