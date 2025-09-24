// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

import java.util.Objects;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class IngestMessageAttemptFailingEvent {
    @JsonProperty private IngestMessageAttemptFailingEventData data;
    @JsonProperty private TypeEnum type;

    public IngestMessageAttemptFailingEvent() {}

    public IngestMessageAttemptFailingEvent data(IngestMessageAttemptFailingEventData data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public IngestMessageAttemptFailingEventData getData() {
        return data;
    }

    public void setData(IngestMessageAttemptFailingEventData data) {
        this.data = data;
    }

    public IngestMessageAttemptFailingEvent type(TypeEnum type) {
        this.type = type;
        return this;
    }

    /**
     * Get type
     *
     * @return type
     */
    @javax.annotation.Nonnull
    public TypeEnum getType() {
        return type;
    }

    public void setType(TypeEnum type) {
        this.type = type;
    }

    public enum TypeEnum {
        INGEST_MESSAGE_ATTEMPT_FAILING("ingest.message.attempt.failing");
        private final String value;

        TypeEnum(String value) {
            this.value = value;
        }

        @JsonValue
        public String getValue() {
            return this.value;
        }

        @Override
        public String toString() {
            return this.value;
        }

        public static TypeEnum fromValue(String value) {
            if (!Objects.equals(value, "ingest.message.attempt.failing")) {
                throw new IllegalArgumentException("Unexpected value '" + value + "'");
            }
            return INGEST_MESSAGE_ATTEMPT_FAILING;
        }
    }

    /**
     * Create an instance of IngestMessageAttemptFailingEvent given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IngestMessageAttemptFailingEvent
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     IngestMessageAttemptFailingEvent
     */
    public static IngestMessageAttemptFailingEvent fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper()
                .readValue(jsonString, IngestMessageAttemptFailingEvent.class);
    }

    /**
     * Convert an instance of IngestMessageAttemptFailingEvent to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
