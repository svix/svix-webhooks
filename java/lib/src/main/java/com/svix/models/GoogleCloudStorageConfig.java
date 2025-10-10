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
public class GoogleCloudStorageConfig {
    @JsonProperty private String bucket;
    @JsonProperty private String credentials;

    public GoogleCloudStorageConfig() {}

    public GoogleCloudStorageConfig bucket(String bucket) {
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

    public GoogleCloudStorageConfig credentials(String credentials) {
        this.credentials = credentials;
        return this;
    }

    /**
     * Google Cloud Credentials JSON Object as a string.
     *
     * @return credentials
     */
    @javax.annotation.Nonnull
    public String getCredentials() {
        return credentials;
    }

    public void setCredentials(String credentials) {
        this.credentials = credentials;
    }

    /**
     * Create an instance of GoogleCloudStorageConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GoogleCloudStorageConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GoogleCloudStorageConfig
     */
    public static GoogleCloudStorageConfig fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GoogleCloudStorageConfig.class);
    }

    /**
     * Convert an instance of GoogleCloudStorageConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
