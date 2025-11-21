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

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MessageAttemptLogEvent {
    @JsonProperty private List<MessageAttemptLog> data;
    @JsonProperty private TypeEnum type;

    public MessageAttemptLogEvent() {}

    public MessageAttemptLogEvent data(List<MessageAttemptLog> data) {
        this.data = data;
        return this;
    }

    public MessageAttemptLogEvent addDataItem(MessageAttemptLog dataItem) {
        if (this.data == null) {
            this.data = new ArrayList<>();
        }
        this.data.add(dataItem);

        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public List<MessageAttemptLog> getData() {
        return data;
    }

    public void setData(List<MessageAttemptLog> data) {
        this.data = data;
    }

    public MessageAttemptLogEvent type(TypeEnum type) {
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
        MESSAGE_ATTEMPT_LOG("message.attempt.log");
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
            if (!Objects.equals(value, "message.attempt.log")) {
                throw new IllegalArgumentException("Unexpected value '" + value + "'");
            }
            return MESSAGE_ATTEMPT_LOG;
        }
    }

    /**
     * Create an instance of MessageAttemptLogEvent given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptLogEvent
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptLogEvent
     */
    public static MessageAttemptLogEvent fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptLogEvent.class);
    }

    /**
     * Convert an instance of MessageAttemptLogEvent to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
