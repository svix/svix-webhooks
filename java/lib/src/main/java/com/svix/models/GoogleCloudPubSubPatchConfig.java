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
public class GoogleCloudPubSubPatchConfig {
    @JsonProperty private String projectId;
    @JsonProperty private String topicId;
    @JsonProperty private String credentials;

    public GoogleCloudPubSubPatchConfig() {}

    public GoogleCloudPubSubPatchConfig projectId(String projectId) {
        this.projectId = projectId;
        return this;
    }

    /**
     * Get projectId
     *
     * @return projectId
     */
    @javax.annotation.Nullable
    public String getProjectId() {
        return projectId;
    }

    public void setProjectId(String projectId) {
        this.projectId = projectId;
    }

    public GoogleCloudPubSubPatchConfig topicId(String topicId) {
        this.topicId = topicId;
        return this;
    }

    /**
     * Get topicId
     *
     * @return topicId
     */
    @javax.annotation.Nullable
    public String getTopicId() {
        return topicId;
    }

    public void setTopicId(String topicId) {
        this.topicId = topicId;
    }

    public GoogleCloudPubSubPatchConfig credentials(String credentials) {
        this.credentials = credentials;
        return this;
    }

    /**
     * Get credentials
     *
     * @return credentials
     */
    @javax.annotation.Nullable
    public String getCredentials() {
        return credentials;
    }

    public void setCredentials(String credentials) {
        this.credentials = credentials;
    }

    /**
     * Create an instance of GoogleCloudPubSubPatchConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GoogleCloudPubSubPatchConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GoogleCloudPubSubPatchConfig
     */
    public static GoogleCloudPubSubPatchConfig fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GoogleCloudPubSubPatchConfig.class);
    }

    /**
     * Convert an instance of GoogleCloudPubSubPatchConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
