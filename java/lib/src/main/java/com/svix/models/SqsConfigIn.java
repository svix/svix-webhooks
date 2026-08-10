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
public class SqsConfigIn {
    @JsonProperty private URI queueUrl;
    @JsonProperty private String region;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private URI endpointUrl;

    public SqsConfigIn() {}

    public SqsConfigIn queueUrl(URI queueUrl) {
        this.queueUrl = queueUrl;
        return this;
    }

    /**
     * Get queueUrl
     *
     * @return queueUrl
     */
    @javax.annotation.Nonnull
    public URI getQueueUrl() {
        return queueUrl;
    }

    public void setQueueUrl(URI queueUrl) {
        this.queueUrl = queueUrl;
    }

    public SqsConfigIn region(String region) {
        this.region = region;
        return this;
    }

    /**
     * Get region
     *
     * @return region
     */
    @javax.annotation.Nonnull
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    public SqsConfigIn accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Get accessKeyId
     *
     * @return accessKeyId
     */
    @javax.annotation.Nonnull
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public SqsConfigIn secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Get secretAccessKey
     *
     * @return secretAccessKey
     */
    @javax.annotation.Nonnull
    public String getSecretAccessKey() {
        return secretAccessKey;
    }

    public void setSecretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
    }

    public SqsConfigIn endpointUrl(URI endpointUrl) {
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
     * Create an instance of SqsConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SqsConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to SqsConfigIn
     */
    public static SqsConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SqsConfigIn.class);
    }

    /**
     * Convert an instance of SqsConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
