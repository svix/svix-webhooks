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
public class SinkTransformationOut {
    @JsonProperty private String code;
    @JsonProperty private Boolean enabled;

    public SinkTransformationOut() {}

    public SinkTransformationOut code(String code) {
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

    public SinkTransformationOut enabled(Boolean enabled) {
        this.enabled = enabled;
        return this;
    }

    /**
     * Get enabled
     *
     * @return enabled
     */
    @javax.annotation.Nonnull
    public Boolean getEnabled() {
        return enabled;
    }

    public void setEnabled(Boolean enabled) {
        this.enabled = enabled;
    }

    /**
     * Create an instance of SinkTransformationOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SinkTransformationOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SinkTransformationOut
     */
    public static SinkTransformationOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkTransformationOut.class);
    }

    /**
     * Convert an instance of SinkTransformationOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
