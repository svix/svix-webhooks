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
public class PandaDocConfig {
    @JsonProperty private String secret;

    public PandaDocConfig() {}

    public PandaDocConfig secret(String secret) {
        this.secret = secret;
        return this;
    }

    /**
     * Get secret
     *
     * @return secret
     */
    @javax.annotation.Nonnull
    public String getSecret() {
        return secret;
    }

    public void setSecret(String secret) {
        this.secret = secret;
    }

    /**
     * Create an instance of PandaDocConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PandaDocConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to PandaDocConfig
     */
    public static PandaDocConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PandaDocConfig.class);
    }

    /**
     * Convert an instance of PandaDocConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
