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
public class GoogleCloudPubSubConfig {
    @JsonProperty private String projectId;
    @JsonProperty private String topicId;
    @JsonProperty private String credentials;

    public GoogleCloudPubSubConfig() {}

    public GoogleCloudPubSubConfig projectId(String projectId) {
        this.projectId = projectId;
        return this;
    }

    /**
     * Get projectId
     *
     * @return projectId
     */
    @javax.annotation.Nonnull
    public String getProjectId() {
        return projectId;
    }

    public void setProjectId(String projectId) {
        this.projectId = projectId;
    }

    public GoogleCloudPubSubConfig topicId(String topicId) {
        this.topicId = topicId;
        return this;
    }

    /**
     * Get topicId
     *
     * @return topicId
     */
    @javax.annotation.Nonnull
    public String getTopicId() {
        return topicId;
    }

    public void setTopicId(String topicId) {
        this.topicId = topicId;
    }

    public GoogleCloudPubSubConfig credentials(String credentials) {
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
     * Create an instance of GoogleCloudPubSubConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GoogleCloudPubSubConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GoogleCloudPubSubConfig
     */
    public static GoogleCloudPubSubConfig fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GoogleCloudPubSubConfig.class);
    }

    /**
     * Convert an instance of GoogleCloudPubSubConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
