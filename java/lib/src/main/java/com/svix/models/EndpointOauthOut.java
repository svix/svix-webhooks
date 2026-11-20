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
public class EndpointOauthOut {
    @JsonProperty private Boolean enabled;
    @JsonProperty private EndpointOauthConfigOut config;

    public EndpointOauthOut() {}

    public EndpointOauthOut enabled(Boolean enabled) {
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

    public EndpointOauthOut config(EndpointOauthConfigOut config) {
        this.config = config;
        return this;
    }

    /**
     * Get config
     *
     * @return config
     */
    @javax.annotation.Nullable
    public EndpointOauthConfigOut getConfig() {
        return config;
    }

    public void setConfig(EndpointOauthConfigOut config) {
        this.config = config;
    }

    /**
     * Create an instance of EndpointOauthOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointOauthOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointOauthOut
     */
    public static EndpointOauthOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointOauthOut.class);
    }

    /**
     * Convert an instance of EndpointOauthOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
