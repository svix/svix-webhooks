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
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class IngestEndpointUpdate {
    @JsonProperty private String description;
    @JsonProperty private Boolean disabled;
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private Long rateLimit;
    @JsonProperty private String uid;
    @JsonProperty private URI url;

    public IngestEndpointUpdate() {}

    public IngestEndpointUpdate description(String description) {
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

    public IngestEndpointUpdate disabled(Boolean disabled) {
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

    public IngestEndpointUpdate metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public IngestEndpointUpdate putMetadataItem(String key, String metadataItem) {
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

    public IngestEndpointUpdate rateLimit(Long rateLimit) {
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

    public IngestEndpointUpdate uid(String uid) {
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

    public IngestEndpointUpdate url(URI url) {
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
     * Create an instance of IngestEndpointUpdate given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IngestEndpointUpdate
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     IngestEndpointUpdate
     */
    public static IngestEndpointUpdate fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IngestEndpointUpdate.class);
    }

    /**
     * Convert an instance of IngestEndpointUpdate to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
