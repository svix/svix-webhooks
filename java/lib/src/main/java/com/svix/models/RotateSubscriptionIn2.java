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
public class RotateSubscriptionIn2 {
    @JsonProperty private EndpointSecretRotateIn signingSecret;

    public RotateSubscriptionIn2() {}

    public RotateSubscriptionIn2 signingSecret(EndpointSecretRotateIn signingSecret) {
        this.signingSecret = signingSecret;
        return this;
    }

    /**
     * Get signingSecret
     *
     * @return signingSecret
     */
    @javax.annotation.Nullable
    public EndpointSecretRotateIn getSigningSecret() {
        return signingSecret;
    }

    public void setSigningSecret(EndpointSecretRotateIn signingSecret) {
        this.signingSecret = signingSecret;
    }

    /**
     * Create an instance of RotateSubscriptionIn2 given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RotateSubscriptionIn2
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RotateSubscriptionIn2
     */
    public static RotateSubscriptionIn2 fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RotateSubscriptionIn2.class);
    }

    /**
     * Convert an instance of RotateSubscriptionIn2 to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
