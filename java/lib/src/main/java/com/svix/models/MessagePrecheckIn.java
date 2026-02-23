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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MessagePrecheckIn {
    @JsonProperty private Set<String> channels;
    @JsonProperty private String eventType;

    public MessagePrecheckIn() {}

    public MessagePrecheckIn channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public MessagePrecheckIn addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new LinkedHashSet<>();
        }
        this.channels.add(channelsItem);

        return this;
    }

    /**
     * Get channels
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

    public MessagePrecheckIn eventType(String eventType) {
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

    /**
     * Create an instance of MessagePrecheckIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessagePrecheckIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessagePrecheckIn
     */
    public static MessagePrecheckIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessagePrecheckIn.class);
    }

    /**
     * Convert an instance of MessagePrecheckIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
