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
public class StreamEventTypeIn {
    @JsonProperty private Boolean archived;
    @JsonProperty private Boolean deprecated;
    @JsonProperty private String description;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String name;

    public StreamEventTypeIn() {}

    public StreamEventTypeIn archived(Boolean archived) {
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

    public StreamEventTypeIn deprecated(Boolean deprecated) {
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

    public StreamEventTypeIn description(String description) {
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

    public StreamEventTypeIn featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public StreamEventTypeIn addFeatureFlagsItem(String featureFlagsItem) {
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

    public StreamEventTypeIn name(String name) {
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

    /**
     * Create an instance of StreamEventTypeIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamEventTypeIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     StreamEventTypeIn
     */
    public static StreamEventTypeIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamEventTypeIn.class);
    }

    /**
     * Convert an instance of StreamEventTypeIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
