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
public class EventBridgeConfigOut {
    @JsonProperty private String eventBusName;
    @JsonProperty private String detailType;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String region;

    public EventBridgeConfigOut() {}

    public EventBridgeConfigOut eventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
        return this;
    }

    /**
     * Get eventBusName
     *
     * @return eventBusName
     */
    @javax.annotation.Nonnull
    public String getEventBusName() {
        return eventBusName;
    }

    public void setEventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
    }

    public EventBridgeConfigOut detailType(String detailType) {
        this.detailType = detailType;
        return this;
    }

    /**
     * Get detailType
     *
     * @return detailType
     */
    @javax.annotation.Nonnull
    public String getDetailType() {
        return detailType;
    }

    public void setDetailType(String detailType) {
        this.detailType = detailType;
    }

    public EventBridgeConfigOut accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Get accessKeyId
     *
     * @return accessKeyId
     */
    @javax.annotation.Nonnull
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public EventBridgeConfigOut region(String region) {
        this.region = region;
        return this;
    }

    /**
     * Get region
     *
     * @return region
     */
    @javax.annotation.Nonnull
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    /**
     * Create an instance of EventBridgeConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventBridgeConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventBridgeConfigOut
     */
    public static EventBridgeConfigOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventBridgeConfigOut.class);
    }

    /**
     * Convert an instance of EventBridgeConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
