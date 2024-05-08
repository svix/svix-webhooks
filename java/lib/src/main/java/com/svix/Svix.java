package com.svix;

import com.svix.internal.ApiClient;
import com.svix.internal.Configuration;
import com.svix.internal.auth.HttpBearerAuth;

public final class Svix {
	public static final String VERSION = "1.24.0";
	private final Application application;
	private final Authentication authentication;
	private final Endpoint endpoint;
	private final EventType eventType;
	private final Integration integration;
	private final Message message;
	private final MessageAttempt messageAttempt;
	private final Statistics statistics;

	public Svix(final String token) {
		this(token, new SvixOptions());
	}

	public Svix(final String token, final SvixOptions options) {
		ApiClient apiClient = new ApiClient();

		String[] tokenParts = token.split("\\.");
		String region = tokenParts[tokenParts.length - 1];
		if (region.equals("us")) {
			apiClient.setBasePath("https://api.us.svix.com");
		} else if (region.equals("eu")) {
			apiClient.setBasePath("https://api.eu.svix.com");
		} else if (region.equals("in")) {
			apiClient.setBasePath("https://api.in.svix.com");
		} else {
			apiClient.setBasePath(options.getServerUrl());
		}

		apiClient.setUserAgent(String.format("svix-libs/%s/java", Svix.VERSION));

		HttpBearerAuth httpBearer = (HttpBearerAuth) apiClient.getAuthentication("HTTPBearer");
		httpBearer.setBearerToken(token);

		Configuration.setDefaultApiClient(apiClient);

		application = new Application();
		authentication = new Authentication();
		endpoint = new Endpoint();
		eventType = new EventType();
		integration = new Integration();
		message = new Message();
		messageAttempt = new MessageAttempt();
		statistics = new Statistics();
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

	public Integration getIntegration() {
		return integration;
	}

	public Message getMessage() {
		return message;
	}

	public MessageAttempt getMessageAttempt() {
		return messageAttempt;
	}

	public Statistics getStatistics() {
		return statistics;
	}
}
