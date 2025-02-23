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
public class EventTypeUpdate {
    @JsonProperty private Boolean archived;
    @JsonProperty private Boolean deprecated;
    @JsonProperty private String description;
    @JsonProperty private String featureFlag;
    @JsonProperty private String groupName;
    @JsonProperty private Object schemas;

    public EventTypeUpdate() {}

    public EventTypeUpdate archived(Boolean archived) {
        this.archived = archived;
        return this;
    }

    /**
     * Get archived
     *
     * @return archived
     */
    @javax.annotation.Nullable
    public Boolean getArchived() {
        return archived;
    }

    public void setArchived(Boolean archived) {
        this.archived = archived;
    }

    public EventTypeUpdate deprecated(Boolean deprecated) {
        this.deprecated = deprecated;
        return this;
    }

    /**
     * Get deprecated
     *
     * @return deprecated
     */
    @javax.annotation.Nullable
    public Boolean getDeprecated() {
        return deprecated;
    }

    public void setDeprecated(Boolean deprecated) {
        this.deprecated = deprecated;
    }

    public EventTypeUpdate description(String description) {
        this.description = description;
        return this;
    }

    /**
     * Get description
     *
     * @return description
     */
    @javax.annotation.Nonnull
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public EventTypeUpdate featureFlag(String featureFlag) {
        this.featureFlag = featureFlag;
        return this;
    }

    /**
     * Get featureFlag
     *
     * @return featureFlag
     */
    @javax.annotation.Nullable
    public String getFeatureFlag() {
        return featureFlag;
    }

    public void setFeatureFlag(String featureFlag) {
        this.featureFlag = featureFlag;
    }

    public EventTypeUpdate groupName(String groupName) {
        this.groupName = groupName;
        return this;
    }

    /**
     * The event type group's name
     *
     * @return groupName
     */
    @javax.annotation.Nullable
    public String getGroupName() {
        return groupName;
    }

    public void setGroupName(String groupName) {
        this.groupName = groupName;
    }

    public EventTypeUpdate schemas(Object schemas) {
        this.schemas = schemas;
        return this;
    }

    /**
     * The schema for the event type for a specific version as a JSON schema.
     *
     * @return schemas
     */
    @javax.annotation.Nullable
    public Object getSchemas() {
        return schemas;
    }

    public void setSchemas(Object schemas) {
        this.schemas = schemas;
    }

    /**
     * Create an instance of EventTypeUpdate given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeUpdate
     * @throws JsonProcessingException if the JSON string is invalid with respect to EventTypeUpdate
     */
    public static EventTypeUpdate fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeUpdate.class);
    }

    /**
     * Convert an instance of EventTypeUpdate to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
