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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EventBridgePatchConfig {
    @JsonProperty private String eventBusName;
    @JsonProperty private String detailType;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private String region;

    public EventBridgePatchConfig() {}

    public EventBridgePatchConfig eventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
        return this;
    }

    /**
     * Get eventBusName
     *
     * @return eventBusName
     */
    @javax.annotation.Nullable
    public String getEventBusName() {
        return eventBusName;
    }

    public void setEventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
    }

    public EventBridgePatchConfig detailType(String detailType) {
        this.detailType = detailType;
        return this;
    }

    /**
     * Get detailType
     *
     * @return detailType
     */
    @javax.annotation.Nullable
    public String getDetailType() {
        return detailType;
    }

    public void setDetailType(String detailType) {
        this.detailType = detailType;
    }

    public EventBridgePatchConfig accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Get accessKeyId
     *
     * @return accessKeyId
     */
    @javax.annotation.Nullable
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public EventBridgePatchConfig secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Get secretAccessKey
     *
     * @return secretAccessKey
     */
    @javax.annotation.Nullable
    public String getSecretAccessKey() {
        return secretAccessKey;
    }

    public void setSecretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
    }

    public EventBridgePatchConfig region(String region) {
        this.region = region;
        return this;
    }

    /**
     * Get region
     *
     * @return region
     */
    @javax.annotation.Nullable
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    /**
     * Create an instance of EventBridgePatchConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventBridgePatchConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventBridgePatchConfig
     */
    public static EventBridgePatchConfig fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventBridgePatchConfig.class);
    }

    /**
     * Convert an instance of EventBridgePatchConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
