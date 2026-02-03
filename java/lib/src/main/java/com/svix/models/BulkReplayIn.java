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
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class BulkReplayIn {
    @JsonProperty private String channel;
    @JsonProperty private Set<String> eventTypes;
    @JsonProperty private OffsetDateTime since;
    @JsonProperty private MessageStatus status;
    @JsonProperty private String tag;
    @JsonProperty private OffsetDateTime until;

    public BulkReplayIn() {}

    public BulkReplayIn channel(String channel) {
        this.channel = channel;
        return this;
    }

    /**
     * Get channel
     *
     * @return channel
     */
    @javax.annotation.Nullable
    public String getChannel() {
        return channel;
    }

    public void setChannel(String channel) {
        this.channel = channel;
    }

    public BulkReplayIn eventTypes(Set<String> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public BulkReplayIn addEventTypesItem(String eventTypesItem) {
        if (this.eventTypes == null) {
            this.eventTypes = new LinkedHashSet<>();
        }
        this.eventTypes.add(eventTypesItem);

        return this;
    }

    /**
     * Get eventTypes
     *
     * @return eventTypes
     */
    @javax.annotation.Nullable
    public Set<String> getEventTypes() {
        return eventTypes;
    }

    public void setEventTypes(Set<String> eventTypes) {
        this.eventTypes = eventTypes;
    }

    public BulkReplayIn since(OffsetDateTime since) {
        this.since = since;
        return this;
    }

    /**
     * Get since
     *
     * @return since
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getSince() {
        return since;
    }

    public void setSince(OffsetDateTime since) {
        this.since = since;
    }

    public BulkReplayIn status(MessageStatus status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nullable
    public MessageStatus getStatus() {
        return status;
    }

    public void setStatus(MessageStatus status) {
        this.status = status;
    }

    public BulkReplayIn tag(String tag) {
        this.tag = tag;
        return this;
    }

    /**
     * Get tag
     *
     * @return tag
     */
    @javax.annotation.Nullable
    public String getTag() {
        return tag;
    }

    public void setTag(String tag) {
        this.tag = tag;
    }

    public BulkReplayIn until(OffsetDateTime until) {
        this.until = until;
        return this;
    }

    /**
     * Get until
     *
     * @return until
     */
    @javax.annotation.Nullable
    public OffsetDateTime getUntil() {
        return until;
    }

    public void setUntil(OffsetDateTime until) {
        this.until = until;
    }

    /**
     * Create an instance of BulkReplayIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BulkReplayIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to BulkReplayIn
     */
    public static BulkReplayIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BulkReplayIn.class);
    }

    /**
     * Convert an instance of BulkReplayIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
