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

import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EventTypeOut {
    @JsonProperty private Boolean archived;
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private Boolean deprecated;
    @JsonProperty private String description;
    @JsonProperty private String featureFlag;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String groupName;
    @JsonProperty private String name;
    @JsonProperty private Object schemas;
    @JsonProperty private OffsetDateTime updatedAt;

    public EventTypeOut() {}

    public EventTypeOut archived(Boolean archived) {
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

    public EventTypeOut createdAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
        return this;
    }

    /**
     * Get createdAt
     *
     * @return createdAt
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public EventTypeOut deprecated(Boolean deprecated) {
        this.deprecated = deprecated;
        return this;
    }

    /**
     * Get deprecated
     *
     * @return deprecated
     */
    @javax.annotation.Nonnull
    public Boolean getDeprecated() {
        return deprecated;
    }

    public void setDeprecated(Boolean deprecated) {
        this.deprecated = deprecated;
    }

    public EventTypeOut description(String description) {
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

    public EventTypeOut featureFlag(String featureFlag) {
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

    public EventTypeOut featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public EventTypeOut addFeatureFlagsItem(String featureFlagsItem) {
        if (this.featureFlags == null) {
            this.featureFlags = new LinkedHashSet<>();
        }
        this.featureFlags.add(featureFlagsItem);

        return this;
    }

    /**
     * Get featureFlags
     *
     * @return featureFlags
     */
    @javax.annotation.Nullable
    public Set<String> getFeatureFlags() {
        return featureFlags;
    }

    public void setFeatureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
    }

    public EventTypeOut groupName(String groupName) {
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

    public EventTypeOut name(String name) {
        this.name = name;
        return this;
    }

    /**
     * The event type's name
     *
     * @return name
     */
    @javax.annotation.Nonnull
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public EventTypeOut schemas(Object schemas) {
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

    public EventTypeOut updatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
        return this;
    }

    /**
     * Get updatedAt
     *
     * @return updatedAt
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    /**
     * Create an instance of EventTypeOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to EventTypeOut
     */
    public static EventTypeOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeOut.class);
    }

    /**
     * Convert an instance of EventTypeOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
