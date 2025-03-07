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
public class RotateTokenOut {
    @JsonProperty private String ingestUrl;

    public RotateTokenOut() {}

    public RotateTokenOut ingestUrl(String ingestUrl) {
        this.ingestUrl = ingestUrl;
        return this;
    }

    /**
     * Get ingestUrl
     *
     * @return ingestUrl
     */
    @javax.annotation.Nonnull
    public String getIngestUrl() {
        return ingestUrl;
    }

    public void setIngestUrl(String ingestUrl) {
        this.ingestUrl = ingestUrl;
    }

    /**
     * Create an instance of RotateTokenOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RotateTokenOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to RotateTokenOut
     */
    public static RotateTokenOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RotateTokenOut.class);
    }

    /**
     * Convert an instance of RotateTokenOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
