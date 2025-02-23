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
import java.util.HashMap;
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class OperationalWebhookEndpointIn {
    @JsonProperty private String description;
    @JsonProperty private Boolean disabled;
    @JsonProperty private Set<String> filterTypes;
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private Long rateLimit;
    @JsonProperty private String secret;
    @JsonProperty private String uid;
    @JsonProperty private URI url;

    public OperationalWebhookEndpointIn() {}

    public OperationalWebhookEndpointIn description(String description) {
        this.description = description;
        return this;
    }

    /**
     * Get description
     *
     * @return description
     */
    @javax.annotation.Nullable
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public OperationalWebhookEndpointIn disabled(Boolean disabled) {
        this.disabled = disabled;
        return this;
    }

    /**
     * Get disabled
     *
     * @return disabled
     */
    @javax.annotation.Nullable
    public Boolean getDisabled() {
        return disabled;
    }

    public void setDisabled(Boolean disabled) {
        this.disabled = disabled;
    }

    public OperationalWebhookEndpointIn filterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
        return this;
    }

    public OperationalWebhookEndpointIn addFilterTypesItem(String filterTypesItem) {
        if (this.filterTypes == null) {
            this.filterTypes = new LinkedHashSet<>();
        }
        this.filterTypes.add(filterTypesItem);

        return this;
    }

    /**
     * Get filterTypes
     *
     * @return filterTypes
     */
    @javax.annotation.Nullable
    public Set<String> getFilterTypes() {
        return filterTypes;
    }

    public void setFilterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
    }

    public OperationalWebhookEndpointIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public OperationalWebhookEndpointIn putMetadataItem(String key, String metadataItem) {
        if (this.metadata == null) {
            this.metadata = new HashMap<>();
        }
        this.metadata.put(key, metadataItem);

        return this;
    }

    /**
     * Get metadata
     *
     * @return metadata
     */
    @javax.annotation.Nullable
    public Map<String, String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String, String> metadata) {
        this.metadata = metadata;
    }

    public OperationalWebhookEndpointIn rateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
        return this;
    }

    /**
     * Get rateLimit
     *
     * @return rateLimit
     */
    @javax.annotation.Nullable
    public Long getRateLimit() {
        return rateLimit;
    }

    public void setRateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
    }

    public OperationalWebhookEndpointIn secret(String secret) {
        this.secret = secret;
        return this;
    }

    /**
     * The endpoint's verification secret.
     *
     * <p>Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended
     * to not set this and let the server generate the secret.
     *
     * @return secret
     */
    @javax.annotation.Nullable
    public String getSecret() {
        return secret;
    }

    public void setSecret(String secret) {
        this.secret = secret;
    }

    public OperationalWebhookEndpointIn uid(String uid) {
        this.uid = uid;
        return this;
    }

    /**
     * Optional unique identifier for the endpoint.
     *
     * @return uid
     */
    @javax.annotation.Nullable
    public String getUid() {
        return uid;
    }

    public void setUid(String uid) {
        this.uid = uid;
    }

    public OperationalWebhookEndpointIn url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    /**
     * Create an instance of OperationalWebhookEndpointIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of OperationalWebhookEndpointIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     OperationalWebhookEndpointIn
     */
    public static OperationalWebhookEndpointIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, OperationalWebhookEndpointIn.class);
    }

    /**
     * Convert an instance of OperationalWebhookEndpointIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
