// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.MaybeUnset;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

import java.net.URI;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class SnsConfigPatch {
    @JsonProperty private String topicArn;
    @JsonProperty private String region;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private MaybeUnset<URI> endpointUrl;

    public SnsConfigPatch() {}

    public SnsConfigPatch topicArn(String topicArn) {
        this.topicArn = topicArn;
        return this;
    }

    /**
     * Get topicArn
     *
     * @return topicArn
     */
    @javax.annotation.Nullable
    public String getTopicArn() {
        return topicArn;
    }

    public void setTopicArn(String topicArn) {
        this.topicArn = topicArn;
    }

    public SnsConfigPatch region(String region) {
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

    public SnsConfigPatch accessKeyId(String accessKeyId) {
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

    public SnsConfigPatch secretAccessKey(String secretAccessKey) {
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

    public SnsConfigPatch endpointUrl(URI endpointUrl) {
        this.endpointUrl = new MaybeUnset<>(endpointUrl);
        return this;
    }

    /**
     * Get endpointUrl
     *
     * @return endpointUrl
     */
    @javax.annotation.Nullable
    public URI getEndpointUrl() {
        if (endpointUrl == null) {
            return null;
        }
        return endpointUrl.getValue();
    }

    public void setEndpointUrl(URI endpointUrl) {
        this.endpointUrl = new MaybeUnset<>(endpointUrl);
    }

    /**
     * Create an instance of SnsConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SnsConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to SnsConfigPatch
     */
    public static SnsConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SnsConfigPatch.class);
    }

    /**
     * Convert an instance of SnsConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
