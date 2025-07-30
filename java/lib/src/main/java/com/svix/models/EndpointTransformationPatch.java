// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.MaybeUnset;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointTransformationPatch {
    @JsonProperty private MaybeUnset<String> code;
    @JsonProperty private Boolean enabled;

    public EndpointTransformationPatch() {}

    public EndpointTransformationPatch code(String code) {
        this.code = new MaybeUnset<>(code);
        return this;
    }

    /**
     * Get code
     *
     * @return code
     */
    @javax.annotation.Nullable
    public String getCode() {
        if (code == null) {
            return null;
        }
        return code.getValue();
    }

    public void setCode(String code) {
        this.code = new MaybeUnset<>(code);
    }

    public EndpointTransformationPatch enabled(Boolean enabled) {
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

    /**
     * Create an instance of EndpointTransformationPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointTransformationPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointTransformationPatch
     */
    public static EndpointTransformationPatch fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointTransformationPatch.class);
    }

    /**
     * Convert an instance of EndpointTransformationPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
