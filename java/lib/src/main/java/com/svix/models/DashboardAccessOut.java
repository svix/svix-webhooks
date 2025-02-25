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
public class DashboardAccessOut {
    @JsonProperty private String token;
    @JsonProperty private URI url;

    public DashboardAccessOut() {}

    public DashboardAccessOut token(String token) {
        this.token = token;
        return this;
    }

    /**
     * Get token
     *
     * @return token
     */
    @javax.annotation.Nonnull
    public String getToken() {
        return token;
    }

    public void setToken(String token) {
        this.token = token;
    }

    public DashboardAccessOut url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    /**
     * Create an instance of DashboardAccessOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of DashboardAccessOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     DashboardAccessOut
     */
    public static DashboardAccessOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, DashboardAccessOut.class);
    }

    /**
     * Convert an instance of DashboardAccessOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
