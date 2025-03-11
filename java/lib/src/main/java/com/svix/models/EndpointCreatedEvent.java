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
public class EndpointCreatedEvent {
    @JsonProperty private EndpointCreatedEventData data;
    @JsonProperty private TypeEnum type;

    public EndpointCreatedEvent() {}

    public EndpointCreatedEvent data(EndpointCreatedEventData data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public EndpointCreatedEventData getData() {
        return data;
    }

    public void setData(EndpointCreatedEventData data) {
        this.data = data;
    }

    public EndpointCreatedEvent type(TypeEnum type) {
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
        ENDPOINT_CREATED("endpoint.created");
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
            if (!Objects.equals(value, "endpoint.created")) {
                throw new IllegalArgumentException("Unexpected value '" + value + "'");
            }
            return ENDPOINT_CREATED;
        }
    }

    /**
     * Create an instance of EndpointCreatedEvent given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointCreatedEvent
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointCreatedEvent
     */
    public static EndpointCreatedEvent fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointCreatedEvent.class);
    }

    /**
     * Convert an instance of EndpointCreatedEvent to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
