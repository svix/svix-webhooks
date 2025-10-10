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
public class RotatePollerTokenIn {
    @JsonProperty private Long expiry;
    @JsonProperty private Long oldTokenExpiry;

    public RotatePollerTokenIn() {}

    public RotatePollerTokenIn expiry(Long expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
     * How long the token will be valid for, in seconds. Can be up to 31,536,000 seconds (1 year).
     *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Long getExpiry() {
        return expiry;
    }

    public void setExpiry(Long expiry) {
        this.expiry = expiry;
    }

    public RotatePollerTokenIn oldTokenExpiry(Long oldTokenExpiry) {
        this.oldTokenExpiry = oldTokenExpiry;
        return this;
    }

    /**
     * Updates the previous token's expiration, in seconds.
     *
     * <p>If set to 0, the old token will immediately be revoked. Must be between 0 and 86,400
     * seconds (1 day).
     *
     * <p>Defaults to 300 seconds (5 minutes).
     *
     * @return oldTokenExpiry
     */
    @javax.annotation.Nullable
    public Long getOldTokenExpiry() {
        return oldTokenExpiry;
    }

    public void setOldTokenExpiry(Long oldTokenExpiry) {
        this.oldTokenExpiry = oldTokenExpiry;
    }

    /**
     * Create an instance of RotatePollerTokenIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RotatePollerTokenIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RotatePollerTokenIn
     */
    public static RotatePollerTokenIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RotatePollerTokenIn.class);
    }

    /**
     * Convert an instance of RotatePollerTokenIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
