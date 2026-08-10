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
public class GoogleCloudStorageConfigOut {
    @JsonProperty private String bucket;

    public GoogleCloudStorageConfigOut() {}

    public GoogleCloudStorageConfigOut bucket(String bucket) {
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

    /**
     * Create an instance of GoogleCloudStorageConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GoogleCloudStorageConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GoogleCloudStorageConfigOut
     */
    public static GoogleCloudStorageConfigOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GoogleCloudStorageConfigOut.class);
    }

    /**
     * Convert an instance of GoogleCloudStorageConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
