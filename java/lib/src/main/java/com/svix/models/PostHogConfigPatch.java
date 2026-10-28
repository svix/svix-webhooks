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

import java.net.URI;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class PostHogConfigPatch {
    @JsonProperty private URI domain;
    @JsonProperty private String projectToken;

    public PostHogConfigPatch() {}

    public PostHogConfigPatch domain(URI domain) {
        this.domain = domain;
        return this;
    }

    /**
     * Get domain
     *
     * @return domain
     */
    @javax.annotation.Nullable
    public URI getDomain() {
        return domain;
    }

    public void setDomain(URI domain) {
        this.domain = domain;
    }

    public PostHogConfigPatch projectToken(String projectToken) {
        this.projectToken = projectToken;
        return this;
    }

    /**
     * Get projectToken
     *
     * @return projectToken
     */
    @javax.annotation.Nullable
    public String getProjectToken() {
        return projectToken;
    }

    public void setProjectToken(String projectToken) {
        this.projectToken = projectToken;
    }

    /**
     * Create an instance of PostHogConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PostHogConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PostHogConfigPatch
     */
    public static PostHogConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PostHogConfigPatch.class);
    }

    /**
     * Convert an instance of PostHogConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
