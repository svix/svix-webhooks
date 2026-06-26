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
public class TailscaleConfig {
    @JsonProperty private String secret;
    @JsonProperty private Integer timestampGraceSeconds;

    public TailscaleConfig() {}

    public TailscaleConfig secret(String secret) {
        this.secret = secret;
        return this;
    }

    /**
     * Shared secret for Tailscale Webhooks
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

    public TailscaleConfig timestampGraceSeconds(Integer timestampGraceSeconds) {
        this.timestampGraceSeconds = timestampGraceSeconds;
        return this;
    }

    /**
     * Grace period (in seconds) for the timestamp.
     *
     * <p>If not passed, timestamp age will not be checked.
     *
     * @return timestampGraceSeconds
     */
    @javax.annotation.Nullable
    public Integer getTimestampGraceSeconds() {
        return timestampGraceSeconds;
    }

    public void setTimestampGraceSeconds(Integer timestampGraceSeconds) {
        this.timestampGraceSeconds = timestampGraceSeconds;
    }

    /**
     * Create an instance of TailscaleConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of TailscaleConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to TailscaleConfig
     */
    public static TailscaleConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, TailscaleConfig.class);
    }

    /**
     * Convert an instance of TailscaleConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
