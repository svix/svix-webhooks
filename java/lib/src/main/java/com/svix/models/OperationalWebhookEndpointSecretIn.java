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
public class OperationalWebhookEndpointSecretIn {
    @JsonProperty private String key;

    public OperationalWebhookEndpointSecretIn() {}

    public OperationalWebhookEndpointSecretIn key(String key) {
        this.key = key;
        return this;
    }

    /**
     * The endpoint's verification secret.
     *
     * <p>Format: `base64` encoded random bytes optionally prefixed with `whsec_`. It is recommended
     * to not set this and let the server generate the secret.
     *
     * @return key
     */
    @javax.annotation.Nullable
    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    /**
     * Create an instance of OperationalWebhookEndpointSecretIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of OperationalWebhookEndpointSecretIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     OperationalWebhookEndpointSecretIn
     */
    public static OperationalWebhookEndpointSecretIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper()
                .readValue(jsonString, OperationalWebhookEndpointSecretIn.class);
    }

    /**
     * Convert an instance of OperationalWebhookEndpointSecretIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
