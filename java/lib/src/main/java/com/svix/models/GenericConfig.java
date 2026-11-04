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
public class GenericConfig {
    @JsonProperty private GenericResponse response;

    public GenericConfig() {}

    public GenericConfig response(GenericResponse response) {
        this.response = response;
        return this;
    }

    /**
     * Get response
     *
     * @return response
     */
    @javax.annotation.Nullable
    public GenericResponse getResponse() {
        return response;
    }

    public void setResponse(GenericResponse response) {
        this.response = response;
    }

    /**
     * Create an instance of GenericConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GenericConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to GenericConfig
     */
    public static GenericConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GenericConfig.class);
    }

    /**
     * Convert an instance of GenericConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
