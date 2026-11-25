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

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointOauthConfigOut {
    @JsonProperty private Oauth2GrantTypeInOut grantType;
    @JsonProperty private Oauth2AuthMethodInOut authMethod;
    @JsonProperty private String clientId;
    @JsonProperty private String tokenUrl;
    @JsonProperty private List<String> scopes;
    @JsonProperty private Map<String, String> extraParams;
    @JsonProperty private Map<String, String> extraHeaders;

    public EndpointOauthConfigOut() {}

    public EndpointOauthConfigOut grantType(Oauth2GrantTypeInOut grantType) {
        this.grantType = grantType;
        return this;
    }

    /**
     * Get grantType
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

    public EndpointOauthConfigOut authMethod(Oauth2AuthMethodInOut authMethod) {
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

    public EndpointOauthConfigOut clientId(String clientId) {
        this.clientId = clientId;
        return this;
    }

    /**
     * Get clientId
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

    public EndpointOauthConfigOut tokenUrl(String tokenUrl) {
        this.tokenUrl = tokenUrl;
        return this;
    }

    /**
     * Get tokenUrl
     *
     * @return tokenUrl
     */
    @javax.annotation.Nonnull
    public String getTokenUrl() {
        return tokenUrl;
    }

    public void setTokenUrl(String tokenUrl) {
        this.tokenUrl = tokenUrl;
    }

    public EndpointOauthConfigOut scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public EndpointOauthConfigOut addScopesItem(String scopesItem) {
        if (this.scopes == null) {
            this.scopes = new ArrayList<>();
        }
        this.scopes.add(scopesItem);

        return this;
    }

    /**
     * Get scopes
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

    public EndpointOauthConfigOut extraParams(Map<String, String> extraParams) {
        this.extraParams = extraParams;
        return this;
    }

    public EndpointOauthConfigOut putExtraParamsItem(String key, String extraParamsItem) {
        if (this.extraParams == null) {
            this.extraParams = new HashMap<>();
        }
        this.extraParams.put(key, extraParamsItem);

        return this;
    }

    /**
     * Get extraParams
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

    public EndpointOauthConfigOut extraHeaders(Map<String, String> extraHeaders) {
        this.extraHeaders = extraHeaders;
        return this;
    }

    public EndpointOauthConfigOut putExtraHeadersItem(String key, String extraHeadersItem) {
        if (this.extraHeaders == null) {
            this.extraHeaders = new HashMap<>();
        }
        this.extraHeaders.put(key, extraHeadersItem);

        return this;
    }

    /**
     * Get extraHeaders
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
     * Create an instance of EndpointOauthConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointOauthConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointOauthConfigOut
     */
    public static EndpointOauthConfigOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointOauthConfigOut.class);
    }

    /**
     * Convert an instance of EndpointOauthConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
