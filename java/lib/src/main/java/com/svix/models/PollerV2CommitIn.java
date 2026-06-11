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
public class PollerV2CommitIn {
    @JsonProperty private Long offset;

    public PollerV2CommitIn() {}

    public PollerV2CommitIn offset(Long offset) {
        this.offset = offset;
        return this;
    }

    /**
     * Get offset
     *
     * @return offset
     */
    @javax.annotation.Nonnull
    public Long getOffset() {
        return offset;
    }

    public void setOffset(Long offset) {
        this.offset = offset;
    }

    /**
     * Create an instance of PollerV2CommitIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PollerV2CommitIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PollerV2CommitIn
     */
    public static PollerV2CommitIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PollerV2CommitIn.class);
    }

    /**
     * Convert an instance of PollerV2CommitIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
