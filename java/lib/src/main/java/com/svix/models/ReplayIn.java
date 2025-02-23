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
public class ReplayIn {
    @JsonProperty private OffsetDateTime since;
    @JsonProperty private OffsetDateTime until;

    public ReplayIn() {}

    public ReplayIn since(OffsetDateTime since) {
        this.since = since;
        return this;
    }

    /**
     * Get since
     *
     * @return since
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getSince() {
        return since;
    }

    public void setSince(OffsetDateTime since) {
        this.since = since;
    }

    public ReplayIn until(OffsetDateTime until) {
        this.until = until;
        return this;
    }

    /**
     * Get until
     *
     * @return until
     */
    @javax.annotation.Nullable
    public OffsetDateTime getUntil() {
        return until;
    }

    public void setUntil(OffsetDateTime until) {
        this.until = until;
    }

    /**
     * Create an instance of ReplayIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ReplayIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to ReplayIn
     */
    public static ReplayIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ReplayIn.class);
    }

    /**
     * Convert an instance of ReplayIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
