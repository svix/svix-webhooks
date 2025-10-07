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
public class MessageOut {
    @JsonProperty private Set<String> channels;
    @JsonProperty private OffsetDateTime deliverAt;
    @JsonProperty private String eventId;
    @JsonProperty private String eventType;
    @JsonProperty private String id;
    @JsonProperty private Object payload;
    @JsonProperty private Set<String> tags;
    @JsonProperty private OffsetDateTime timestamp;

    public MessageOut() {}

    public MessageOut channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public MessageOut addChannelsItem(String channelsItem) {
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

    public MessageOut deliverAt(OffsetDateTime deliverAt) {
        this.deliverAt = deliverAt;
        return this;
    }

    /**
     * Get deliverAt
     *
     * @return deliverAt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getDeliverAt() {
        return deliverAt;
    }

    public void setDeliverAt(OffsetDateTime deliverAt) {
        this.deliverAt = deliverAt;
    }

    public MessageOut eventId(String eventId) {
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

    public MessageOut eventType(String eventType) {
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

    public MessageOut id(String id) {
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

    public MessageOut payload(Object payload) {
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

    public MessageOut tags(Set<String> tags) {
        this.tags = tags;
        return this;
    }

    public MessageOut addTagsItem(String tagsItem) {
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

    public MessageOut timestamp(OffsetDateTime timestamp) {
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
     * Create an instance of MessageOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to MessageOut
     */
    public static MessageOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageOut.class);
    }

    /**
     * Convert an instance of MessageOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
