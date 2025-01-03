package com.svix;

import com.svix.internal.ApiClient;
import com.svix.internal.Configuration;
import com.svix.internal.auth.HttpBearerAuth;

public final class Svix {
	public static final String VERSION = "1.56.0";
	private final Application application;
	private final Authentication authentication;
	private final Endpoint endpoint;
	private final EventType eventType;
	private final Integration integration;
	private final Message message;
	private final MessageAttempt messageAttempt;
	private final Statistics statistics;
	private final OperationalWebhookEndpoint operationalWebhookEndpoint;

	public Svix(final String token) {
		this(token, new SvixOptions());
	}

	public Svix(final String token, final SvixOptions options) {
		ApiClient apiClient = new ApiClient();

		if (options.hasServerUrl()) {
			apiClient.setBasePath(options.getServerUrl());
		} else {
			String[] tokenParts = token.split("\\.");
			String region = tokenParts[tokenParts.length - 1];
			if (region.equals("us")) {
				apiClient.setBasePath("https://api.us.svix.com");
			} else if (region.equals("eu")) {
				apiClient.setBasePath("https://api.eu.svix.com");
			} else if (region.equals("in")) {
				apiClient.setBasePath("https://api.in.svix.com");
			} else {
				// Fallback when all else fails.
				apiClient.setBasePath("https://api.svix.com");
			}
		}

		apiClient.setUserAgent(String.format("svix-libs/%s/java", Svix.VERSION));

		HttpBearerAuth httpBearer = (HttpBearerAuth) apiClient.getAuthentication("HTTPBearer");
		httpBearer.setBearerToken(token);

		application = new Application(apiClient);
		authentication = new Authentication(apiClient);
		endpoint = new Endpoint(apiClient);
		eventType = new EventType(apiClient);
		integration = new Integration(apiClient);
		message = new Message(apiClient);
		messageAttempt = new MessageAttempt(apiClient);
		statistics = new Statistics(apiClient);
		operationalWebhookEndpoint = new OperationalWebhookEndpoint(apiClient);
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

	public OperationalWebhookEndpoint getOperationalWebhookEndpoint() {
		return operationalWebhookEndpoint;
	}
}
