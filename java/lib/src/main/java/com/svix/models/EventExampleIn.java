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
public class EventExampleIn {
    @JsonProperty private String eventType;
    @JsonProperty private Long exampleIndex;

    public EventExampleIn() {}

    public EventExampleIn eventType(String eventType) {
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

    public EventExampleIn exampleIndex(Long exampleIndex) {
        this.exampleIndex = exampleIndex;
        return this;
    }

    /**
     * If the event type schema contains an array of examples, chooses which one to send.
     *
     * <p>Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
     *
     * @return exampleIndex
     */
    @javax.annotation.Nullable
    public Long getExampleIndex() {
        return exampleIndex;
    }

    public void setExampleIndex(Long exampleIndex) {
        this.exampleIndex = exampleIndex;
    }

    /**
     * Create an instance of EventExampleIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventExampleIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to EventExampleIn
     */
    public static EventExampleIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventExampleIn.class);
    }

    /**
     * Convert an instance of EventExampleIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
