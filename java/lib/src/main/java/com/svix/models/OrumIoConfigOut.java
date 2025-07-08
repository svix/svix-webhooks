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
public class OrumIoConfigOut {
    @JsonProperty private String publicKey;

    public OrumIoConfigOut() {}

    public OrumIoConfigOut publicKey(String publicKey) {
        this.publicKey = publicKey;
        return this;
    }

    /**
     * Get publicKey
     *
     * @return publicKey
     */
    @javax.annotation.Nonnull
    public String getPublicKey() {
        return publicKey;
    }

    public void setPublicKey(String publicKey) {
        this.publicKey = publicKey;
    }

    /**
     * Create an instance of OrumIoConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of OrumIoConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to OrumIoConfigOut
     */
    public static OrumIoConfigOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, OrumIoConfigOut.class);
    }

    /**
     * Convert an instance of OrumIoConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
