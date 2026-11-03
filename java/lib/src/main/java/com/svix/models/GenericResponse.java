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
public class GenericResponse {
    @JsonProperty private String contentType;
    @JsonProperty private String body;

    public GenericResponse() {}

    public GenericResponse contentType(String contentType) {
        this.contentType = contentType;
        return this;
    }

    /**
     * Get contentType
     *
     * @return contentType
     */
    @javax.annotation.Nonnull
    public String getContentType() {
        return contentType;
    }

    public void setContentType(String contentType) {
        this.contentType = contentType;
    }

    public GenericResponse body(String body) {
        this.body = body;
        return this;
    }

    /**
     * Get body
     *
     * @return body
     */
    @javax.annotation.Nonnull
    public String getBody() {
        return body;
    }

    public void setBody(String body) {
        this.body = body;
    }

    /**
     * Create an instance of GenericResponse given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GenericResponse
     * @throws JsonProcessingException if the JSON string is invalid with respect to GenericResponse
     */
    public static GenericResponse fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GenericResponse.class);
    }

    /**
     * Convert an instance of GenericResponse to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
