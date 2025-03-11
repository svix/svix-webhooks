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
public class MessageAttemptExhaustedEvent {
    @JsonProperty private MessageAttemptExhaustedEventData data;
    @JsonProperty private TypeEnum type;

    public MessageAttemptExhaustedEvent() {}

    public MessageAttemptExhaustedEvent data(MessageAttemptExhaustedEventData data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public MessageAttemptExhaustedEventData getData() {
        return data;
    }

    public void setData(MessageAttemptExhaustedEventData data) {
        this.data = data;
    }

    public MessageAttemptExhaustedEvent type(TypeEnum type) {
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
        MESSAGE_ATTEMPT_EXHAUSTED("message.attempt.exhausted");
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
            if (!Objects.equals(value, "message.attempt.exhausted")) {
                throw new IllegalArgumentException("Unexpected value '" + value + "'");
            }
            return MESSAGE_ATTEMPT_EXHAUSTED;
        }
    }

    /**
     * Create an instance of MessageAttemptExhaustedEvent given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptExhaustedEvent
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptExhaustedEvent
     */
    public static MessageAttemptExhaustedEvent fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptExhaustedEvent.class);
    }

    /**
     * Convert an instance of MessageAttemptExhaustedEvent to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
