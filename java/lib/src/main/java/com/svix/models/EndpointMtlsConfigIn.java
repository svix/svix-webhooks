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
public class EndpointMtlsConfigIn {
    @JsonProperty private String identity;

    public EndpointMtlsConfigIn() {}

    public EndpointMtlsConfigIn identity(String identity) {
        this.identity = identity;
        return this;
    }

    /**
     * A PEM encoded private key and X.509 certificate to identify the webhook sender.
     *
     * @return identity
     */
    @javax.annotation.Nonnull
    public String getIdentity() {
        return identity;
    }

    public void setIdentity(String identity) {
        this.identity = identity;
    }

    /**
     * Create an instance of EndpointMtlsConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointMtlsConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointMtlsConfigIn
     */
    public static EndpointMtlsConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointMtlsConfigIn.class);
    }

    /**
     * Convert an instance of EndpointMtlsConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
