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
public class EndpointDeletedEvent {
    @JsonProperty private EndpointDeletedEventData data;
    @JsonProperty private TypeEnum type;

    public EndpointDeletedEvent() {}

    public EndpointDeletedEvent data(EndpointDeletedEventData data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public EndpointDeletedEventData getData() {
        return data;
    }

    public void setData(EndpointDeletedEventData data) {
        this.data = data;
    }

    public EndpointDeletedEvent type(TypeEnum type) {
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
        ENDPOINT_DELETED("endpoint.deleted");
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
            if (!Objects.equals(value, "endpoint.deleted")) {
                throw new IllegalArgumentException("Unexpected value '" + value + "'");
            }
            return ENDPOINT_DELETED;
        }
    }

    /**
     * Create an instance of EndpointDeletedEvent given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointDeletedEvent
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointDeletedEvent
     */
    public static EndpointDeletedEvent fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointDeletedEvent.class);
    }

    /**
     * Convert an instance of EndpointDeletedEvent to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
