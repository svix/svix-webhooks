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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointDisabledEventData {
    @JsonProperty private String appId;
    @JsonProperty private String appUid;
    @JsonProperty private String endpointId;
    @JsonProperty private String endpointUid;
    @JsonProperty private OffsetDateTime failSince;
    @JsonProperty private EndpointDisabledTrigger trigger;

    public EndpointDisabledEventData() {}

    public EndpointDisabledEventData appId(String appId) {
        this.appId = appId;
        return this;
    }

    /**
     * The Application's ID.
     *
     * @return appId
     */
    @javax.annotation.Nonnull
    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }

    public EndpointDisabledEventData appUid(String appUid) {
        this.appUid = appUid;
        return this;
    }

    /**
     * The Application's UID.
     *
     * @return appUid
     */
    @javax.annotation.Nullable
    public String getAppUid() {
        return appUid;
    }

    public void setAppUid(String appUid) {
        this.appUid = appUid;
    }

    public EndpointDisabledEventData endpointId(String endpointId) {
        this.endpointId = endpointId;
        return this;
    }

    /**
     * The Endpoint's ID.
     *
     * @return endpointId
     */
    @javax.annotation.Nonnull
    public String getEndpointId() {
        return endpointId;
    }

    public void setEndpointId(String endpointId) {
        this.endpointId = endpointId;
    }

    public EndpointDisabledEventData endpointUid(String endpointUid) {
        this.endpointUid = endpointUid;
        return this;
    }

    /**
     * The Endpoint's UID.
     *
     * @return endpointUid
     */
    @javax.annotation.Nullable
    public String getEndpointUid() {
        return endpointUid;
    }

    public void setEndpointUid(String endpointUid) {
        this.endpointUid = endpointUid;
    }

    public EndpointDisabledEventData failSince(OffsetDateTime failSince) {
        this.failSince = failSince;
        return this;
    }

    /**
     * Get failSince
     *
     * @return failSince
     */
    @javax.annotation.Nullable
    public OffsetDateTime getFailSince() {
        return failSince;
    }

    public void setFailSince(OffsetDateTime failSince) {
        this.failSince = failSince;
    }

    public EndpointDisabledEventData trigger(EndpointDisabledTrigger trigger) {
        this.trigger = trigger;
        return this;
    }

    /**
     * Get trigger
     *
     * @return trigger
     */
    @javax.annotation.Nullable
    public EndpointDisabledTrigger getTrigger() {
        return trigger;
    }

    public void setTrigger(EndpointDisabledTrigger trigger) {
        this.trigger = trigger;
    }

    /**
     * Create an instance of EndpointDisabledEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointDisabledEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointDisabledEventData
     */
    public static EndpointDisabledEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointDisabledEventData.class);
    }

    /**
     * Convert an instance of EndpointDisabledEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
