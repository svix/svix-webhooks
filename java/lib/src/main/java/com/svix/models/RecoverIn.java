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
public class RecoverIn {
    @JsonProperty private OffsetDateTime since;
    @JsonProperty private OffsetDateTime until;

    public RecoverIn() {}

    public RecoverIn since(OffsetDateTime since) {
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

    public RecoverIn until(OffsetDateTime until) {
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
     * Create an instance of RecoverIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RecoverIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to RecoverIn
     */
    public static RecoverIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RecoverIn.class);
    }

    /**
     * Convert an instance of RecoverIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
