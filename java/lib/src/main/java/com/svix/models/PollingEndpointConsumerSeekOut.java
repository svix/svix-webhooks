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
public class PollingEndpointConsumerSeekOut {
    @JsonProperty private String iterator;

    public PollingEndpointConsumerSeekOut() {}

    public PollingEndpointConsumerSeekOut iterator(String iterator) {
        this.iterator = iterator;
        return this;
    }

    /**
     * Get iterator
     *
     * @return iterator
     */
    @javax.annotation.Nonnull
    public String getIterator() {
        return iterator;
    }

    public void setIterator(String iterator) {
        this.iterator = iterator;
    }

    /**
     * Create an instance of PollingEndpointConsumerSeekOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PollingEndpointConsumerSeekOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PollingEndpointConsumerSeekOut
     */
    public static PollingEndpointConsumerSeekOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PollingEndpointConsumerSeekOut.class);
    }

    /**
     * Convert an instance of PollingEndpointConsumerSeekOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
