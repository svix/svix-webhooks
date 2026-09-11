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
public class SnsConfigIn {
    @JsonProperty private String topicArn;
    @JsonProperty private String region;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private URI endpointUrl;
    @JsonProperty private String roleArn;
    @JsonProperty private String externalId;

    public SnsConfigIn() {}

    public SnsConfigIn topicArn(String topicArn) {
        this.topicArn = topicArn;
        return this;
    }

    /**
     * Get topicArn
     *
     * @return topicArn
     */
    @javax.annotation.Nonnull
    public String getTopicArn() {
        return topicArn;
    }

    public void setTopicArn(String topicArn) {
        this.topicArn = topicArn;
    }

    public SnsConfigIn region(String region) {
        this.region = region;
        return this;
    }

    /**
     * The region of the SNS instance.
     *
     * <p>Currently a required field, but marked as optional because we may infer it from other
     * fields in the future.
     *
     * @return region
     */
    @javax.annotation.Nullable
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    public SnsConfigIn accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Access key ID.
     *
     * <p>Required (along with `secret_access_key`) if `role_arn` is None
     *
     * @return accessKeyId
     */
    @javax.annotation.Nullable
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public SnsConfigIn secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Secret access key.
     *
     * <p>Required (along with `access_key_id`) if `role_arn` is None
     *
     * @return secretAccessKey
     */
    @javax.annotation.Nullable
    public String getSecretAccessKey() {
        return secretAccessKey;
    }

    public void setSecretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
    }

    public SnsConfigIn endpointUrl(URI endpointUrl) {
        this.endpointUrl = endpointUrl;
        return this;
    }

    /**
     * Get endpointUrl
     *
     * @return endpointUrl
     */
    @javax.annotation.Nullable
    public URI getEndpointUrl() {
        return endpointUrl;
    }

    public void setEndpointUrl(URI endpointUrl) {
        this.endpointUrl = endpointUrl;
    }

    public SnsConfigIn roleArn(String roleArn) {
        this.roleArn = roleArn;
        return this;
    }

    /**
     * Role ARN for delegated authentication
     *
     * @return roleArn
     */
    @javax.annotation.Nullable
    public String getRoleArn() {
        return roleArn;
    }

    public void setRoleArn(String roleArn) {
        this.roleArn = roleArn;
    }

    public SnsConfigIn externalId(String externalId) {
        this.externalId = externalId;
        return this;
    }

    /**
     * Shared secret passed as the STS ExternalId.
     *
     * <p>Can only be set if `role_arn` is Some
     *
     * @return externalId
     */
    @javax.annotation.Nullable
    public String getExternalId() {
        return externalId;
    }

    public void setExternalId(String externalId) {
        this.externalId = externalId;
    }

    /**
     * Create an instance of SnsConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SnsConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to SnsConfigIn
     */
    public static SnsConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SnsConfigIn.class);
    }

    /**
     * Convert an instance of SnsConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
