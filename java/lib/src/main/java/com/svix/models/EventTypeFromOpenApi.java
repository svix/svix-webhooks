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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EventTypeFromOpenApi {
    @JsonProperty private Boolean deprecated;
    @JsonProperty private String description;
    @JsonProperty private String featureFlag;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String groupName;
    @JsonProperty private String name;
    @JsonProperty private Object schemas;

    public EventTypeFromOpenApi() {}

    public EventTypeFromOpenApi deprecated(Boolean deprecated) {
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

    public EventTypeFromOpenApi description(String description) {
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

    @Deprecated
    public EventTypeFromOpenApi featureFlag(String featureFlag) {
        this.featureFlag = featureFlag;
        return this;
    }

    /**
     * Deprecated, use `featureFlags` instead.
     *
     * @return featureFlag
     */
    @javax.annotation.Nullable
    @Deprecated
    public String getFeatureFlag() {
        return featureFlag;
    }

    @Deprecated
    public void setFeatureFlag(String featureFlag) {
        this.featureFlag = featureFlag;
    }

    public EventTypeFromOpenApi featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public EventTypeFromOpenApi addFeatureFlagsItem(String featureFlagsItem) {
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

    public EventTypeFromOpenApi groupName(String groupName) {
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

    public EventTypeFromOpenApi name(String name) {
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

    public EventTypeFromOpenApi schemas(Object schemas) {
        this.schemas = schemas;
        return this;
    }

    /**
     * Get schemas
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
     * Create an instance of EventTypeFromOpenApi given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeFromOpenApi
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventTypeFromOpenApi
     */
    public static EventTypeFromOpenApi fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeFromOpenApi.class);
    }

    /**
     * Convert an instance of EventTypeFromOpenApi to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
