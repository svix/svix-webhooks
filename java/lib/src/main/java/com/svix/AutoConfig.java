package com.svix;

import com.fasterxml.jackson.databind.JsonNode;
import com.svix.exceptions.ApiException;
import com.svix.exceptions.EmptyWebhookSecretException;
import com.svix.exceptions.WebhookVerificationException;
import com.svix.internalapi.EndpointAutoConfigDeprecated;
import com.svix.internalapi.EndpointAutoconfig;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.SubscribeIn;
import java.io.IOException;
import java.util.Base64;
import java.util.List;
import java.util.Map;

public final class AutoConfig {
  static final String AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";
  static final String AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_";
  static final String UNSUPPORTED_TOKEN_VERSION =
      "Unsupported token version. You might need to update the Svix SDK to use this token";

  private final String appId;
  private final String endpointId;
  private final String autoconfigId;
  private final EndpointIn endpoint;
  private final Webhook webhook;
  private final Svix svix;

  public AutoConfig(final String token, final EndpointIn endpointIn) throws InvalidTokenException {
    DecodedTokenContent content = decodeToken(token);
    this.appId = content.getAppId();
    this.endpointId = content.getEndpointId();
    this.autoconfigId = content.getAutoconfigId();
    this.endpoint = endpointIn;
    try {
      this.webhook = new Webhook(content.getEndpointSecret());
    } catch (IllegalArgumentException e) {
      throw new InvalidTokenException(e);
    } catch (EmptyWebhookSecretException e) {
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

  public String getAutoconfigId() {
    return autoconfigId;
  }

  public EndpointIn getEndpoint() {
    return endpoint;
  }

  /** Registers this endpoint with Svix using the auto-config API. */
  public EndpointOut subscribe() throws IOException, ApiException {
    if (autoconfigId != null) {
      return new EndpointAutoconfig(svix.getHttpClient()).subscribe(appId, autoconfigId,
          endpoint);
    }
    return new EndpointAutoConfigDeprecated(svix.getHttpClient()).update(appId, endpointId,
        new SubscribeIn().endpoint(endpoint));
  }

  public void verify(final String payload, final Map<String, List<String>> headers)
      throws WebhookVerificationException {
    webhook.verify(payload, headers);
  }

  /**
   * Parses and validates an auto-config token.
   *
   * @throws InvalidTokenException if the token is missing the expected prefix, is not valid Base64,
   *         or does not contain a valid JSON payload with the required fields.
   */
  static DecodedTokenContent decodeToken(final String token) throws InvalidTokenException {
    if (token != null && token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
      return decodeTokenV1(token);
    }
    if (token != null && token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V2)) {
      return decodeTokenV2(token);
    }
    throw new InvalidTokenException(UNSUPPORTED_TOKEN_VERSION);
  }

  static DecodedTokenContent decodeTokenV1(final String token) throws InvalidTokenException {
    JsonNode node = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V1);
    return new DecodedTokenContent(requiredText(node, "aid"), requiredText(node, "eid"), null,
        requiredText(node, "surl"), requiredText(node, "esec"), requiredText(node, "tok"));
  }

  static DecodedTokenContent decodeTokenV2(final String token) throws InvalidTokenException {
    JsonNode node = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V2);
    return new DecodedTokenContent(requiredText(node, "aid"), null, requiredText(node, "sid"),
        requiredText(node, "surl"), requiredText(node, "esec"), requiredText(node, "tok"));
  }

  private static JsonNode parseTokenPayload(final String token, final String prefix)
      throws InvalidTokenException {
    if (token == null || !token.startsWith(prefix)) {
      throw new InvalidTokenException(UNSUPPORTED_TOKEN_VERSION);
    }

    final byte[] decoded;
    try {
      decoded = Base64.getDecoder().decode(token.substring(prefix.length()));
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
    return node;
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
    private final String autoconfigId;
    private final String serverUrl;
    private final String endpointSecret;
    private final String tokenPlaintext;

    private DecodedTokenContent(final String appId, final String endpointId,
        final String autoconfigId, final String serverUrl, final String endpointSecret,
        final String tokenPlaintext) {
      this.appId = appId;
      this.endpointId = endpointId;
      this.autoconfigId = autoconfigId;
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

    public String getAutoconfigId() {
      return autoconfigId;
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

    public InvalidTokenException(final String detail) {
      super(detail);
    }

    public InvalidTokenException(final Throwable cause) {
      super("invalid token", cause);
    }
  }
}
