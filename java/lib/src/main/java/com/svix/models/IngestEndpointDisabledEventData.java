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
public class IngestEndpointDisabledEventData {
    @JsonProperty private String endpointId;
    @JsonProperty private String endpointUid;
    @JsonProperty private OffsetDateTime failSince;
    @JsonProperty private String sourceId;
    @JsonProperty private EndpointDisabledTrigger trigger;

    public IngestEndpointDisabledEventData() {}

    public IngestEndpointDisabledEventData endpointId(String endpointId) {
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

    public IngestEndpointDisabledEventData endpointUid(String endpointUid) {
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

    public IngestEndpointDisabledEventData failSince(OffsetDateTime failSince) {
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

    public IngestEndpointDisabledEventData sourceId(String sourceId) {
        this.sourceId = sourceId;
        return this;
    }

    /**
     * The Source's ID.
     *
     * @return sourceId
     */
    @javax.annotation.Nonnull
    public String getSourceId() {
        return sourceId;
    }

    public void setSourceId(String sourceId) {
        this.sourceId = sourceId;
    }

    public IngestEndpointDisabledEventData trigger(EndpointDisabledTrigger trigger) {
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
     * Create an instance of IngestEndpointDisabledEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IngestEndpointDisabledEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     IngestEndpointDisabledEventData
     */
    public static IngestEndpointDisabledEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IngestEndpointDisabledEventData.class);
    }

    /**
     * Convert an instance of IngestEndpointDisabledEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
