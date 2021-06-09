package com.svix;

import com.svix.internal.ApiClient;
import com.svix.internal.Configuration;
import com.svix.internal.auth.HttpBearerAuth;

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

	public Application application() {
		return application;
	}

	public Authentication authentication() {
		return authentication;
	}

	public Endpoint endpoint() {
		return endpoint;
	}

	public EventType eventType() {
		return eventType;
	}

	public Message message() {
		return message;
	}

	public MessageAttempt messageAttempt() {
		return messageAttempt;
	}
}
