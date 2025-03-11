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
public class MessageAttemptFailedData {
    @JsonProperty private String id;
    @JsonProperty private Short responseStatusCode;
    @JsonProperty private OffsetDateTime timestamp;

    public MessageAttemptFailedData() {}

    public MessageAttemptFailedData id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The MessageAttempt's ID.
     *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public MessageAttemptFailedData responseStatusCode(Short responseStatusCode) {
        this.responseStatusCode = responseStatusCode;
        return this;
    }

    /**
     * Get responseStatusCode
     *
     * @return responseStatusCode
     */
    @javax.annotation.Nonnull
    public Short getResponseStatusCode() {
        return responseStatusCode;
    }

    public void setResponseStatusCode(Short responseStatusCode) {
        this.responseStatusCode = responseStatusCode;
    }

    public MessageAttemptFailedData timestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
     * Get timestamp
     *
     * @return timestamp
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
    }

    /**
     * Create an instance of MessageAttemptFailedData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptFailedData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptFailedData
     */
    public static MessageAttemptFailedData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptFailedData.class);
    }

    /**
     * Convert an instance of MessageAttemptFailedData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
