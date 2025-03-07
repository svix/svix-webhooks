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
public class CronConfig {
    @JsonProperty private String contentType;
    @JsonProperty private String payload;
    @JsonProperty private String schedule;

    public CronConfig() {}

    public CronConfig contentType(String contentType) {
        this.contentType = contentType;
        return this;
    }

    /**
     * Override the default content-type.
     *
     * <p>Recommended if the payload is not JSON.
     *
     * @return contentType
     */
    @javax.annotation.Nullable
    public String getContentType() {
        return contentType;
    }

    public void setContentType(String contentType) {
        this.contentType = contentType;
    }

    public CronConfig payload(String payload) {
        this.payload = payload;
        return this;
    }

    /**
     * Get payload
     *
     * @return payload
     */
    @javax.annotation.Nonnull
    public String getPayload() {
        return payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public CronConfig schedule(String schedule) {
        this.schedule = schedule;
        return this;
    }

    /**
     * Get schedule
     *
     * @return schedule
     */
    @javax.annotation.Nonnull
    public String getSchedule() {
        return schedule;
    }

    public void setSchedule(String schedule) {
        this.schedule = schedule;
    }

    /**
     * Create an instance of CronConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CronConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to CronConfig
     */
    public static CronConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CronConfig.class);
    }

    /**
     * Convert an instance of CronConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
