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

import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class CreateStreamEventsIn {
    @JsonProperty private List<EventIn> events;
    @JsonProperty private StreamIn stream;

    public CreateStreamEventsIn() {}

    public CreateStreamEventsIn events(List<EventIn> events) {
        this.events = events;
        return this;
    }

    public CreateStreamEventsIn addEventsItem(EventIn eventsItem) {
        if (this.events == null) {
            this.events = new ArrayList<>();
        }
        this.events.add(eventsItem);

        return this;
    }

    /**
     * Get events
     *
     * @return events
     */
    @javax.annotation.Nonnull
    public List<EventIn> getEvents() {
        return events;
    }

    public void setEvents(List<EventIn> events) {
        this.events = events;
    }

    public CreateStreamEventsIn stream(StreamIn stream) {
        this.stream = stream;
        return this;
    }

    /**
     * Optionally creates a new Stream alongside the events.
     *
     * <p>If the stream id or uid that is used in the path already exists, this argument is ignored.
     *
     * @return stream
     */
    @javax.annotation.Nullable
    public StreamIn getStream() {
        return stream;
    }

    public void setStream(StreamIn stream) {
        this.stream = stream;
    }

    /**
     * Create an instance of CreateStreamEventsIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CreateStreamEventsIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     CreateStreamEventsIn
     */
    public static CreateStreamEventsIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CreateStreamEventsIn.class);
    }

    /**
     * Convert an instance of CreateStreamEventsIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
