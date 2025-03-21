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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class PollingEndpointConsumerSeekIn {
    @JsonProperty private OffsetDateTime after;

    public PollingEndpointConsumerSeekIn() {}

    public PollingEndpointConsumerSeekIn after(OffsetDateTime after) {
        this.after = after;
        return this;
    }

    /**
     * Get after
     *
     * @return after
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getAfter() {
        return after;
    }

    public void setAfter(OffsetDateTime after) {
        this.after = after;
    }

    /**
     * Create an instance of PollingEndpointConsumerSeekIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PollingEndpointConsumerSeekIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PollingEndpointConsumerSeekIn
     */
    public static PollingEndpointConsumerSeekIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PollingEndpointConsumerSeekIn.class);
    }

    /**
     * Convert an instance of PollingEndpointConsumerSeekIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
