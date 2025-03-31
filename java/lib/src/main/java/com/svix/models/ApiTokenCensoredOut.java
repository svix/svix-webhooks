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

import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ApiTokenCensoredOut {
    @JsonProperty private String censoredToken;
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private OffsetDateTime expiresAt;
    @JsonProperty private String id;
    @JsonProperty private String name;
    @JsonProperty private List<String> scopes;

    public ApiTokenCensoredOut() {}

    public ApiTokenCensoredOut censoredToken(String censoredToken) {
        this.censoredToken = censoredToken;
        return this;
    }

    /**
     * Get censoredToken
     *
     * @return censoredToken
     */
    @javax.annotation.Nonnull
    public String getCensoredToken() {
        return censoredToken;
    }

    public void setCensoredToken(String censoredToken) {
        this.censoredToken = censoredToken;
    }

    public ApiTokenCensoredOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    /**
     * Get createdAt
     *
     * @return createdAt
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public ApiTokenCensoredOut expiresAt(OffsetDateTime expiresAt) {
        this.expiresAt = expiresAt;
        return this;
    }

    /**
     * Get expiresAt
     *
     * @return expiresAt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getExpiresAt() {
        return expiresAt;
    }

    public void setExpiresAt(OffsetDateTime expiresAt) {
        this.expiresAt = expiresAt;
    }

    public ApiTokenCensoredOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The ApplicationToken's ID.
     *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public ApiTokenCensoredOut name(String name) {
        this.name = name;
        return this;
    }

    /**
     * Get name
     *
     * @return name
     */
    @javax.annotation.Nullable
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public ApiTokenCensoredOut scopes(List<String> scopes) {
        this.scopes = scopes;
        return this;
    }

    public ApiTokenCensoredOut addScopesItem(String scopesItem) {
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

    /**
     * Create an instance of ApiTokenCensoredOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ApiTokenCensoredOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ApiTokenCensoredOut
     */
    public static ApiTokenCensoredOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ApiTokenCensoredOut.class);
    }

    /**
     * Convert an instance of ApiTokenCensoredOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
