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
public class EndpointMessageOut {
    @JsonProperty private Set<String> channels;
    @JsonProperty private String eventId;
    @JsonProperty private String eventType;
    @JsonProperty private String id;
    @JsonProperty private OffsetDateTime nextAttempt;
    @JsonProperty private Object payload;
    @JsonProperty private MessageStatus status;
    @JsonProperty private MessageStatusText statusText;
    @JsonProperty private Set<String> tags;
    @JsonProperty private OffsetDateTime timestamp;

    public EndpointMessageOut() {}

    public EndpointMessageOut channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public EndpointMessageOut addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new LinkedHashSet<>();
        }
        this.channels.add(channelsItem);

        return this;
    }

    /**
     * List of free-form identifiers that endpoints can filter by
     *
     * @return channels
     */
    @javax.annotation.Nullable
    public Set<String> getChannels() {
        return channels;
    }

    public void setChannels(Set<String> channels) {
        this.channels = channels;
    }

    public EndpointMessageOut eventId(String eventId) {
        this.eventId = eventId;
        return this;
    }

    /**
     * Optional unique identifier for the message
     *
     * @return eventId
     */
    @javax.annotation.Nullable
    public String getEventId() {
        return eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public EndpointMessageOut eventType(String eventType) {
        this.eventType = eventType;
        return this;
    }

    /**
     * The event type's name
     *
     * @return eventType
     */
    @javax.annotation.Nonnull
    public String getEventType() {
        return eventType;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public EndpointMessageOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The Message's ID.
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

    public EndpointMessageOut nextAttempt(OffsetDateTime nextAttempt) {
        this.nextAttempt = nextAttempt;
        return this;
    }

    /**
     * Get nextAttempt
     *
     * @return nextAttempt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getNextAttempt() {
        return nextAttempt;
    }

    public void setNextAttempt(OffsetDateTime nextAttempt) {
        this.nextAttempt = nextAttempt;
    }

    public EndpointMessageOut payload(Object payload) {
        this.payload = payload;
        return this;
    }

    /**
     * Get payload
     *
     * @return payload
     */
    @javax.annotation.Nonnull
    public Object getPayload() {
        return payload;
    }

    public void setPayload(Object payload) {
        this.payload = payload;
    }

    public EndpointMessageOut status(MessageStatus status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nonnull
    public MessageStatus getStatus() {
        return status;
    }

    public void setStatus(MessageStatus status) {
        this.status = status;
    }

    public EndpointMessageOut statusText(MessageStatusText statusText) {
        this.statusText = statusText;
        return this;
    }

    /**
     * Get statusText
     *
     * @return statusText
     */
    @javax.annotation.Nonnull
    public MessageStatusText getStatusText() {
        return statusText;
    }

    public void setStatusText(MessageStatusText statusText) {
        this.statusText = statusText;
    }

    public EndpointMessageOut tags(Set<String> tags) {
        this.tags = tags;
        return this;
    }

    public EndpointMessageOut addTagsItem(String tagsItem) {
        if (this.tags == null) {
            this.tags = new LinkedHashSet<>();
        }
        this.tags.add(tagsItem);

        return this;
    }

    /**
     * Get tags
     *
     * @return tags
     */
    @javax.annotation.Nullable
    public Set<String> getTags() {
        return tags;
    }

    public void setTags(Set<String> tags) {
        this.tags = tags;
    }

    public EndpointMessageOut timestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
     * Get timestamp
     *
     * @return timestamp
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
    }

    /**
     * Create an instance of EndpointMessageOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointMessageOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointMessageOut
     */
    public static EndpointMessageOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointMessageOut.class);
    }

    /**
     * Convert an instance of EndpointMessageOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
