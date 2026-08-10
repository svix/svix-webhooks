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
public class GoogleCloudPubSubConfigIn {
    @JsonProperty private String projectId;
    @JsonProperty private String topicId;
    @JsonProperty private String credentials;

    public GoogleCloudPubSubConfigIn() {}

    public GoogleCloudPubSubConfigIn projectId(String projectId) {
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

    public GoogleCloudPubSubConfigIn topicId(String topicId) {
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

    public GoogleCloudPubSubConfigIn credentials(String credentials) {
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
     * Create an instance of GoogleCloudPubSubConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GoogleCloudPubSubConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GoogleCloudPubSubConfigIn
     */
    public static GoogleCloudPubSubConfigIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GoogleCloudPubSubConfigIn.class);
    }

    /**
     * Convert an instance of GoogleCloudPubSubConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
