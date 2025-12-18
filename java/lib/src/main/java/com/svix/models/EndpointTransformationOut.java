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
public class EndpointTransformationOut {
    @JsonProperty private String code;
    @JsonProperty private Boolean enabled;
    @JsonProperty private OffsetDateTime updatedAt;

    public EndpointTransformationOut() {}

    public EndpointTransformationOut code(String code) {
        this.code = code;
        return this;
    }

    /**
     * Get code
     *
     * @return code
     */
    @javax.annotation.Nullable
    public String getCode() {
        return code;
    }

    public void setCode(String code) {
        this.code = code;
    }

    public EndpointTransformationOut enabled(Boolean enabled) {
        this.enabled = enabled;
        return this;
    }

    /**
     * Get enabled
     *
     * @return enabled
     */
    @javax.annotation.Nullable
    public Boolean getEnabled() {
        return enabled;
    }

    public void setEnabled(Boolean enabled) {
        this.enabled = enabled;
    }

    public EndpointTransformationOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    /**
     * Get updatedAt
     *
     * @return updatedAt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    /**
     * Create an instance of EndpointTransformationOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointTransformationOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointTransformationOut
     */
    public static EndpointTransformationOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointTransformationOut.class);
    }

    /**
     * Convert an instance of EndpointTransformationOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
