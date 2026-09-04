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
public class S3ConfigIn {
    @JsonProperty private String bucket;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private String region;
    @JsonProperty private URI endpointUrl;
    @JsonProperty private String roleArn;
    @JsonProperty private String externalId;

    public S3ConfigIn() {}

    public S3ConfigIn bucket(String bucket) {
        this.bucket = bucket;
        return this;
    }

    /**
     * Get bucket
     *
     * @return bucket
     */
    @javax.annotation.Nonnull
    public String getBucket() {
        return bucket;
    }

    public void setBucket(String bucket) {
        this.bucket = bucket;
    }

    public S3ConfigIn accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Access key ID.
     *
     * <p>Required (along with `secret_access_key`) if `role_arn` is null
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

    public S3ConfigIn secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Secret access key.
     *
     * <p>Required (along with `access_key_id`) if `role_arn` is null
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

    public S3ConfigIn region(String region) {
        this.region = region;
        return this;
    }

    /**
     * The region of the S3 bucket
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

    public S3ConfigIn endpointUrl(URI endpointUrl) {
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

    public S3ConfigIn roleArn(String roleArn) {
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

    public S3ConfigIn externalId(String externalId) {
        this.externalId = externalId;
        return this;
    }

    /**
     * Shared secret passed as the STS ExternalId.
     *
     * <p>Recommended if `role_arn` is not null.
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
     * Create an instance of S3ConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of S3ConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to S3ConfigIn
     */
    public static S3ConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, S3ConfigIn.class);
    }

    /**
     * Convert an instance of S3ConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
