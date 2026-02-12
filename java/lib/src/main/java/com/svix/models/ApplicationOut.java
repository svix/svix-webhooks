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
import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ApplicationOut {
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private String id;
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private String name;
    @JsonProperty private Long rateLimit;
    @JsonProperty private Long throttleRate;
    @JsonProperty private String uid;
    @JsonProperty private OffsetDateTime updatedAt;

    public ApplicationOut() {}

    public ApplicationOut createdAt(OffsetDateTime createdAt) {
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

    public ApplicationOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The Application's ID.
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

    public ApplicationOut metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public ApplicationOut putMetadataItem(String key, String metadataItem) {
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
    @javax.annotation.Nonnull
    public Map<String, String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String, String> metadata) {
        this.metadata = metadata;
    }

    public ApplicationOut name(String name) {
        this.name = name;
        return this;
    }

    /**
     * Application name for human consumption.
     *
     * @return name
     */
    @javax.annotation.Nonnull
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    @Deprecated
    public ApplicationOut rateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
        return this;
    }

    /**
     * Deprecated, use `throttleRate` instead.
     *
     * @return rateLimit
     */
    @javax.annotation.Nullable
    @Deprecated
    public Long getRateLimit() {
        return rateLimit;
    }

    @Deprecated
    public void setRateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
    }

    public ApplicationOut throttleRate(Long throttleRate) {
        this.throttleRate = throttleRate;
        return this;
    }

    /**
     * Maximum messages per second to send to this application.
     *
     * <p>Outgoing messages will be throttled to this rate.
     *
     * @return throttleRate
     */
    @javax.annotation.Nullable
    public Long getThrottleRate() {
        return throttleRate;
    }

    public void setThrottleRate(Long throttleRate) {
        this.throttleRate = throttleRate;
    }

    public ApplicationOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    /**
     * Optional unique identifier for the application.
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

    public ApplicationOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    /**
     * Get updatedAt
     *
     * @return updatedAt
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    /**
     * Create an instance of ApplicationOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ApplicationOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ApplicationOut
     */
    public static ApplicationOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ApplicationOut.class);
    }

    /**
     * Convert an instance of ApplicationOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
