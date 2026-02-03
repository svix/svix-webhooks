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

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ApplicationIn {
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private String name;
    @JsonProperty private Long rateLimit;
    @JsonProperty private Long throttleRate;
    @JsonProperty private String uid;

    public ApplicationIn() {}

    public ApplicationIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public ApplicationIn putMetadataItem(String key, String metadataItem) {
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

    public ApplicationIn name(String name) {
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
    public ApplicationIn rateLimit(Long rateLimit) {
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

    public ApplicationIn throttleRate(Long throttleRate) {
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

    public ApplicationIn uid(String uid) {
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

    /**
     * Create an instance of ApplicationIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ApplicationIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to ApplicationIn
     */
    public static ApplicationIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ApplicationIn.class);
    }

    /**
     * Convert an instance of ApplicationIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
