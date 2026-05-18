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

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointTransformationPatch {
    @JsonProperty private MaybeUnset<String> code;
    @JsonProperty private Boolean enabled;
    @JsonProperty private MaybeUnset<Map<String, String>> variables;

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

    public EndpointTransformationPatch variables(Map<String, String> variables) {
        this.variables = new MaybeUnset<>(variables);
        return this;
    }

    public EndpointTransformationPatch putVariablesItem(String key, String variablesItem) {
        if (this.variables == null) {
            this.variables = new MaybeUnset<>(new HashMap<>());
        }
        this.variables.getValue().put(key, variablesItem);

        return this;
    }

    /**
     * Get variables
     *
     * @return variables
     */
    @javax.annotation.Nullable
    public Map<String, String> getVariables() {
        if (variables == null) {
            return null;
        }
        return variables.getValue();
    }

    public void setVariables(Map<String, String> variables) {
        this.variables = new MaybeUnset<>(variables);
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
