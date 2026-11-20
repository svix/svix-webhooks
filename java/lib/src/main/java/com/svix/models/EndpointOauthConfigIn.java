// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

import java.net.URI;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointOauthConfigIn {
    @JsonProperty private Oauth2GrantTypeInOut grantType;
    @JsonProperty private Oauth2AuthMethodInOut authMethod;
    @JsonProperty private ClientSecretJwtParamsIn jwtParams;
    @JsonProperty private String clientId;
    @JsonProperty private String clientSecret;
    @JsonProperty private URI tokenUrl;
    @JsonProperty private List<String> scopes;
    @JsonProperty private Map<String, String> extraParams;
    @JsonProperty private String refreshToken;
    @JsonProperty private Map<String, String> extraHeaders;

    public EndpointOauthConfigIn() {}

    public EndpointOauthConfigIn grantType(Oauth2GrantTypeInOut grantType) {
        this.grantType = grantType;
        return this;
    }

    /**
     * The OAuth grant type.
     *
     * @return grantType
     */
    @javax.annotation.Nonnull
    public Oauth2GrantTypeInOut getGrantType() {
        return grantType;
    }

    public void setGrantType(Oauth2GrantTypeInOut grantType) {
        this.grantType = grantType;
    }

    public EndpointOauthConfigIn authMethod(Oauth2AuthMethodInOut authMethod) {
        this.authMethod = authMethod;
        return this;
    }

    /**
     * Get authMethod
     *
     * @return authMethod
     */
    @javax.annotation.Nonnull
    public Oauth2AuthMethodInOut getAuthMethod() {
        return authMethod;
    }

    public void setAuthMethod(Oauth2AuthMethodInOut authMethod) {
        this.authMethod = authMethod;
    }

    public EndpointOauthConfigIn jwtParams(ClientSecretJwtParamsIn jwtParams) {
        this.jwtParams = jwtParams;
        return this;
    }

    /**
     * Optional JWT parameters. Only required for `clientSecretJwt`
     *
     * @return jwtParams
     */
    @javax.annotation.Nullable
    public ClientSecretJwtParamsIn getJwtParams() {
        return jwtParams;
    }

    public void setJwtParams(ClientSecretJwtParamsIn jwtParams) {
        this.jwtParams = jwtParams;
    }

    public EndpointOauthConfigIn clientId(String clientId) {
        this.clientId = clientId;
        return this;
    }

    /**
     * The client ID. Required for all authentication types.
     *
     * @return clientId
     */
    @javax.annotation.Nonnull
    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public EndpointOauthConfigIn clientSecret(String clientSecret) {
        this.clientSecret = clientSecret;
        return this;
    }

    /**
     * Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
     *
     * <p>For `clientSecretBasic`, the secret will be appended to the `Authorization` header. For
     * `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
     *
     * @return clientSecret
     */
    @javax.annotation.Nullable
    public String getClientSecret() {
        return clientSecret;
    }

    public void setClientSecret(String clientSecret) {
        this.clientSecret = clientSecret;
    }

    public EndpointOauthConfigIn tokenUrl(URI tokenUrl) {
        this.tokenUrl = tokenUrl;
        return this;
    }

    /**
     * The URL of the authorization server.
     *
     * @return tokenUrl
     */
    @javax.annotation.Nonnull
    public URI getTokenUrl() {
        return tokenUrl;
    }

    public void setTokenUrl(URI tokenUrl) {
        this.tokenUrl = tokenUrl;
    }

    public EndpointOauthConfigIn scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public EndpointOauthConfigIn addScopesItem(String scopesItem) {
        if (this.scopes == null) {
            this.scopes = new ArrayList<>();
        }
        this.scopes.add(scopesItem);

        return this;
    }

    /**
     * Optional OAuth scopes added to the request body.
     *
     * @return scopes
     */
    @javax.annotation.Nullable
    public List<String> getScopes() {
        return scopes;
    }

    public void setScopes(List<String> scopes) {
        this.scopes = scopes;
    }

    public EndpointOauthConfigIn extraParams(Map<String, String> extraParams) {
        this.extraParams = extraParams;
        return this;
    }

    public EndpointOauthConfigIn putExtraParamsItem(String key, String extraParamsItem) {
        if (this.extraParams == null) {
            this.extraParams = new HashMap<>();
        }
        this.extraParams.put(key, extraParamsItem);

        return this;
    }

    /**
     * Extra parameters added to the request body as key-value pairs.
     *
     * @return extraParams
     */
    @javax.annotation.Nullable
    public Map<String, String> getExtraParams() {
        return extraParams;
    }

    public void setExtraParams(Map<String, String> extraParams) {
        this.extraParams = extraParams;
    }

    public EndpointOauthConfigIn refreshToken(String refreshToken) {
        this.refreshToken = refreshToken;
        return this;
    }

    /**
     * For `refreshToken` grant type.
     *
     * @return refreshToken
     */
    @javax.annotation.Nullable
    public String getRefreshToken() {
        return refreshToken;
    }

    public void setRefreshToken(String refreshToken) {
        this.refreshToken = refreshToken;
    }

    public EndpointOauthConfigIn extraHeaders(Map<String, String> extraHeaders) {
        this.extraHeaders = extraHeaders;
        return this;
    }

    public EndpointOauthConfigIn putExtraHeadersItem(String key, String extraHeadersItem) {
        if (this.extraHeaders == null) {
            this.extraHeaders = new HashMap<>();
        }
        this.extraHeaders.put(key, extraHeadersItem);

        return this;
    }

    /**
     * Optional headers added to the token request.
     *
     * @return extraHeaders
     */
    @javax.annotation.Nullable
    public Map<String, String> getExtraHeaders() {
        return extraHeaders;
    }

    public void setExtraHeaders(Map<String, String> extraHeaders) {
        this.extraHeaders = extraHeaders;
    }

    /**
     * Create an instance of EndpointOauthConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointOauthConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointOauthConfigIn
     */
    public static EndpointOauthConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointOauthConfigIn.class);
    }

    /**
     * Convert an instance of EndpointOauthConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
