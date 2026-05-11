package com.svix;

import com.fasterxml.jackson.databind.JsonNode;
import com.svix.exceptions.ApiException;
import com.svix.exceptions.WebhookVerificationException;
import com.svix.internalapi.EndpointAutoConfig;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.SubscribeIn;

import java.io.IOException;
import java.util.Base64;
import java.util.List;
import java.util.Map;

public final class AutoConfig {
    public static final String AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";

    private final String appId;
    private final String endpointId;
    private final EndpointIn endpoint;
    private final Webhook webhook;
    private final Svix svix;

    public AutoConfig(final String token, final EndpointIn endpointIn) throws InvalidTokenException {
        DecodedTokenContent content = decodeToken(token);
        this.appId = content.getAppId();
        this.endpointId = content.getEndpointId();
        this.endpoint = endpointIn;
        try {
            this.webhook = new Webhook(content.getEndpointSecret());
        } catch (IllegalArgumentException e) {
            throw new InvalidTokenException(e);
        }

        try {
            SvixOptions options = new SvixOptions();
            options.setServerUrl(content.getServerUrl());
            this.svix = new Svix(content.getTokenPlaintext(), options);
        } catch (IllegalArgumentException e) {
            throw new InvalidTokenException(e);
        }
    }

    public String getAppId() {
        return appId;
    }

    public String getEndpointId() {
        return endpointId;
    }

    public EndpointIn getEndpoint() {
        return endpoint;
    }

    /** Registers this endpoint with Svix using the auto-config API. */
    public EndpointOut subscribe() throws IOException, ApiException {
        return new EndpointAutoConfig(svix.getHttpClient())
                .update(appId, endpointId, new SubscribeIn().endpoint(endpoint));
    }

    public void verify(final String payload, final Map<String, List<String>> headers)
            throws WebhookVerificationException {
        webhook.verify(payload, headers);
    }

    /**
     * Parses and validates an auto-config token.
     *
     * @throws InvalidTokenException if the token is missing the expected prefix, is not valid Base64,
     *     or does not contain a valid JSON payload with the required fields.
     */
    public static DecodedTokenContent decodeToken(final String token) throws InvalidTokenException {
        if (token == null || !token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
            throw new InvalidTokenException();
        }

        final byte[] decoded;
        try {
            decoded =
                    Base64.getDecoder()
                            .decode(token.substring(AUTOCONFIG_TOKEN_PREFIX_V1.length()));
        } catch (IllegalArgumentException e) {
            throw new InvalidTokenException(e);
        }

        final JsonNode node;
        try {
            node = Utils.getObjectMapper().readTree(decoded);
        } catch (IOException e) {
            throw new InvalidTokenException(e);
        }

        if (node == null || !node.isObject()) {
            throw new InvalidTokenException();
        }

        String appId = requiredText(node, "aid");
        String endpointId = requiredText(node, "eid");
        String serverUrl = requiredText(node, "surl");
        String endpointSecret = requiredText(node, "esec");
        String tokenPlaintext = requiredText(node, "tok");

        return new DecodedTokenContent(
                appId, endpointId, serverUrl, endpointSecret, tokenPlaintext);
    }

    private static String requiredText(final JsonNode node, final String field)
            throws InvalidTokenException {
        JsonNode v = node.get(field);
        if (v == null || v.isNull() || !v.isTextual()) {
            throw new InvalidTokenException();
        }
        return v.asText();
    }

    public static final class DecodedTokenContent {
        private final String appId;
        private final String endpointId;
        private final String serverUrl;
        private final String endpointSecret;
        private final String tokenPlaintext;

        private DecodedTokenContent(
                final String appId,
                final String endpointId,
                final String serverUrl,
                final String endpointSecret,
                final String tokenPlaintext) {
            this.appId = appId;
            this.endpointId = endpointId;
            this.serverUrl = serverUrl;
            this.endpointSecret = endpointSecret;
            this.tokenPlaintext = tokenPlaintext;
        }

        public String getAppId() {
            return appId;
        }

        public String getEndpointId() {
            return endpointId;
        }

        public String getServerUrl() {
            return serverUrl;
        }

        public String getEndpointSecret() {
            return endpointSecret;
        }

        public String getTokenPlaintext() {
            return tokenPlaintext;
        }
    }

    public static final class InvalidTokenException extends Exception {
        public InvalidTokenException() {
            super("invalid token");
        }

        public InvalidTokenException(final Throwable cause) {
            super("invalid token", cause);
        }
    }
}
