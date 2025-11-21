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
public class HttpAttemptTimes {
    @JsonProperty private OffsetDateTime end;
    @JsonProperty private OffsetDateTime start;

    public HttpAttemptTimes() {}

    public HttpAttemptTimes end(OffsetDateTime end) {
        this.end = end;
        return this;
    }

    /**
     * Get end
     *
     * @return end
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getEnd() {
        return end;
    }

    public void setEnd(OffsetDateTime end) {
        this.end = end;
    }

    public HttpAttemptTimes start(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Get start
     *
     * @return start
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getStart() {
        return start;
    }

    public void setStart(OffsetDateTime start) {
        this.start = start;
    }

    /**
     * Create an instance of HttpAttemptTimes given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of HttpAttemptTimes
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     HttpAttemptTimes
     */
    public static HttpAttemptTimes fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, HttpAttemptTimes.class);
    }

    /**
     * Convert an instance of HttpAttemptTimes to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
