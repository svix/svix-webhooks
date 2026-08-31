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
public class S3ConfigPatch {
    @JsonProperty private String bucket;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private String roleArn;
    @JsonProperty private String externalId;
    @JsonProperty private String region;
    @JsonProperty private URI endpointUrl;

    public S3ConfigPatch() {}

    public S3ConfigPatch bucket(String bucket) {
        this.bucket = bucket;
        return this;
    }

    /**
     * Get bucket
     *
     * @return bucket
     */
    @javax.annotation.Nullable
    public String getBucket() {
        return bucket;
    }

    public void setBucket(String bucket) {
        this.bucket = bucket;
    }

    public S3ConfigPatch accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Get accessKeyId
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

    public S3ConfigPatch secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Get secretAccessKey
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

    public S3ConfigPatch roleArn(String roleArn) {
        this.roleArn = roleArn;
        return this;
    }

    /**
     * Get roleArn
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

    public S3ConfigPatch externalId(String externalId) {
        this.externalId = externalId;
        return this;
    }

    /**
     * Get externalId
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

    public S3ConfigPatch region(String region) {
        this.region = region;
        return this;
    }

    /**
     * Get region
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

    public S3ConfigPatch endpointUrl(URI endpointUrl) {
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

    /**
     * Create an instance of S3ConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of S3ConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to S3ConfigPatch
     */
    public static S3ConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, S3ConfigPatch.class);
    }

    /**
     * Convert an instance of S3ConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
