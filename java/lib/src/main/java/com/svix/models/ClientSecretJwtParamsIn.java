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
public class ClientSecretJwtParamsIn {
    @JsonProperty private String secretBase64;
    @JsonProperty private OauthJwsSigningAlgorithm signingAlgorithm;
    @JsonProperty private String secretId;
    @JsonProperty private Long tokenExpirySecs;

    public ClientSecretJwtParamsIn() {}

    public ClientSecretJwtParamsIn secretBase64(String secretBase64) {
        this.secretBase64 = secretBase64;
        return this;
    }

    /**
     * The base64-encoded secret used for signing the JWT.
     *
     * @return secretBase64
     */
    @javax.annotation.Nonnull
    public String getSecretBase64() {
        return secretBase64;
    }

    public void setSecretBase64(String secretBase64) {
        this.secretBase64 = secretBase64;
    }

    public ClientSecretJwtParamsIn signingAlgorithm(OauthJwsSigningAlgorithm signingAlgorithm) {
        this.signingAlgorithm = signingAlgorithm;
        return this;
    }

    /**
     * The algorithm used to sign the JWT.
     *
     * @return signingAlgorithm
     */
    @javax.annotation.Nonnull
    public OauthJwsSigningAlgorithm getSigningAlgorithm() {
        return signingAlgorithm;
    }

    public void setSigningAlgorithm(OauthJwsSigningAlgorithm signingAlgorithm) {
        this.signingAlgorithm = signingAlgorithm;
    }

    public ClientSecretJwtParamsIn secretId(String secretId) {
        this.secretId = secretId;
        return this;
    }

    /**
     * Optional secret identifier.
     *
     * <p>If supplied, this will be populated in the JWT header in the `kid` field.
     *
     * @return secretId
     */
    @javax.annotation.Nullable
    public String getSecretId() {
        return secretId;
    }

    public void setSecretId(String secretId) {
        this.secretId = secretId;
    }

    public ClientSecretJwtParamsIn tokenExpirySecs(Long tokenExpirySecs) {
        this.tokenExpirySecs = tokenExpirySecs;
        return this;
    }

    /**
     * Optional number of seconds after which the JWT should expire.
     *
     * <p>Defaults to 300 seconds.
     *
     * @return tokenExpirySecs
     */
    @javax.annotation.Nullable
    public Long getTokenExpirySecs() {
        return tokenExpirySecs;
    }

    public void setTokenExpirySecs(Long tokenExpirySecs) {
        this.tokenExpirySecs = tokenExpirySecs;
    }

    /**
     * Create an instance of ClientSecretJwtParamsIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClientSecretJwtParamsIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ClientSecretJwtParamsIn
     */
    public static ClientSecretJwtParamsIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClientSecretJwtParamsIn.class);
    }

    /**
     * Convert an instance of ClientSecretJwtParamsIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
