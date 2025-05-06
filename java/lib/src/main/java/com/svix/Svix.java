package com.svix;

import com.svix.api.*;

import lombok.Getter;

import okhttp3.HttpUrl;

import java.util.Map;

@Getter
public class Svix {
    private final Application application;
    private final Authentication authentication;
    private final BackgroundTask backgroundTask;
    private final Endpoint endpoint;
    private final EventType eventType;
    private final Ingest ingest;
    private final Integration integration;
    private final Management management;
    private final Message message;
    private final MessageAttempt messageAttempt;
    private final Statistics statistics;
    private final OperationalWebhook operationalWebhook;
    private final OperationalWebhookEndpoint operationalWebhookEndpoint;

    public Svix(String token) {
        this(token, new SvixOptions());
    }

    public Svix(String token, SvixOptions options) {
        String[] tokenParts = token.split("\\.");

        if (options.getServerUrl() == null) {
            String region = tokenParts[tokenParts.length - 1];
            switch (region) {
                case "us":
                    options.setServerUrl("https://api.us.svix.com");
                    break;
                case "eu":
                    options.setServerUrl("https://api.eu.svix.com");
                    break;
                case "in":
                    options.setServerUrl("https://api.in.svix.com");
                    break;
                case "ca":
                    options.setServerUrl("https://api.ca.svix.com");
                    break;
                case "au":
                    options.setServerUrl("https://api.au.svix.com");
                    break;

                default:
                    options.setServerUrl(SvixOptions.DEFAULT_URL);
            }
        }

        HttpUrl parsedUrl = HttpUrl.parse(options.getServerUrl());
        if (parsedUrl == null) {
            throw new IllegalArgumentException("Invalid base url");
        }

        Map<String, String> defaultHeaders =
                Map.of(
                        "User-Agent", "svix-libs/" + Version.VERSION + "/java",
                        "Authorization", "Bearer " + token);

        SvixHttpClient httpClient =
                new SvixHttpClient(parsedUrl, defaultHeaders, options.getRetrySchedule());

        this.application = new Application(httpClient);
        this.authentication = new Authentication(httpClient);
        this.backgroundTask = new BackgroundTask(httpClient);
        this.endpoint = new Endpoint(httpClient);
        this.eventType = new EventType(httpClient);
        this.ingest = new Ingest(httpClient);
        this.integration = new Integration(httpClient);
        this.management = new Management(httpClient);
        this.message = new Message(httpClient);
        this.messageAttempt = new MessageAttempt(httpClient);
        this.statistics = new Statistics(httpClient);
        this.operationalWebhook = new OperationalWebhook(httpClient);
        this.operationalWebhookEndpoint = new OperationalWebhookEndpoint(httpClient);
    }
}
