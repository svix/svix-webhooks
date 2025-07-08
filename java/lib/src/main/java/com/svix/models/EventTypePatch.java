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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EventTypePatch {
    @JsonProperty private Boolean archived;
    @JsonProperty private Boolean deprecated;
    @JsonProperty private String description;
    @JsonProperty private MaybeUnset<String> featureFlag;
    @JsonProperty private MaybeUnset<Set<String>> featureFlags;
    @JsonProperty private MaybeUnset<String> groupName;
    @JsonProperty private MaybeUnset<Object> schemas;

    public EventTypePatch() {}

    public EventTypePatch archived(Boolean archived) {
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

    public EventTypePatch deprecated(Boolean deprecated) {
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

    public EventTypePatch description(String description) {
        this.description = description;
        return this;
    }

    /**
     * Get description
     *
     * @return description
     */
    @javax.annotation.Nullable
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    @Deprecated
    public EventTypePatch featureFlag(String featureFlag) {
        this.featureFlag = new MaybeUnset<>(featureFlag);
        return this;
    }

    /**
     * Get featureFlag
     *
     * @return featureFlag
     */
    @javax.annotation.Nullable
    @Deprecated
    public String getFeatureFlag() {
        if (featureFlag == null) {
            return null;
        }
        return featureFlag.getValue();
    }

    @Deprecated
    public void setFeatureFlag(String featureFlag) {
        this.featureFlag = new MaybeUnset<>(featureFlag);
    }

    public EventTypePatch featureFlags(Set<String> featureFlags) {
        this.featureFlags = new MaybeUnset<>(featureFlags);
        return this;
    }

    public EventTypePatch addFeatureFlagsItem(String featureFlagsItem) {
        if (this.featureFlags == null) {
            this.featureFlags = new MaybeUnset<>(new LinkedHashSet<>());
        }
        this.featureFlags.getValue().add(featureFlagsItem);

        return this;
    }

    /**
     * Get featureFlags
     *
     * @return featureFlags
     */
    @javax.annotation.Nullable
    public Set<String> getFeatureFlags() {
        if (featureFlags == null) {
            return null;
        }
        return featureFlags.getValue();
    }

    public void setFeatureFlags(Set<String> featureFlags) {
        this.featureFlags = new MaybeUnset<>(featureFlags);
    }

    public EventTypePatch groupName(String groupName) {
        this.groupName = new MaybeUnset<>(groupName);
        return this;
    }

    /**
     * The event type group's name
     *
     * @return groupName
     */
    @javax.annotation.Nullable
    public String getGroupName() {
        if (groupName == null) {
            return null;
        }
        return groupName.getValue();
    }

    public void setGroupName(String groupName) {
        this.groupName = new MaybeUnset<>(groupName);
    }

    public EventTypePatch schemas(Object schemas) {
        this.schemas = new MaybeUnset<>(schemas);
        return this;
    }

    /**
     * Get schemas
     *
     * @return schemas
     */
    @javax.annotation.Nullable
    public Object getSchemas() {
        if (schemas == null) {
            return null;
        }
        return schemas.getValue();
    }

    public void setSchemas(Object schemas) {
        this.schemas = new MaybeUnset<>(schemas);
    }

    /**
     * Create an instance of EventTypePatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypePatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to EventTypePatch
     */
    public static EventTypePatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypePatch.class);
    }

    /**
     * Convert an instance of EventTypePatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
