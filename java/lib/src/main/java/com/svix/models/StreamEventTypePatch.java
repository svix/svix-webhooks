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
public class StreamEventTypePatch {
    @JsonProperty private Boolean archived;
    @JsonProperty private Boolean deprecated;
    @JsonProperty private MaybeUnset<String> description;
    @JsonProperty private MaybeUnset<Set<String>> featureFlags;
    @JsonProperty private MaybeUnset<String> name;

    public StreamEventTypePatch() {}

    public StreamEventTypePatch archived(Boolean archived) {
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

    public StreamEventTypePatch deprecated(Boolean deprecated) {
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

    public StreamEventTypePatch description(String description) {
        this.description = new MaybeUnset<>(description);
        return this;
    }

    /**
     * Get description
     *
     * @return description
     */
    @javax.annotation.Nullable
    public String getDescription() {
        if (description == null) {
            return null;
        }
        return description.getValue();
    }

    public void setDescription(String description) {
        this.description = new MaybeUnset<>(description);
    }

    public StreamEventTypePatch featureFlags(Set<String> featureFlags) {
        this.featureFlags = new MaybeUnset<>(featureFlags);
        return this;
    }

    public StreamEventTypePatch addFeatureFlagsItem(String featureFlagsItem) {
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

    public StreamEventTypePatch name(String name) {
        this.name = new MaybeUnset<>(name);
        return this;
    }

    /**
     * The event type's name
     *
     * @return name
     */
    @javax.annotation.Nullable
    public String getName() {
        if (name == null) {
            return null;
        }
        return name.getValue();
    }

    public void setName(String name) {
        this.name = new MaybeUnset<>(name);
    }

    /**
     * Create an instance of StreamEventTypePatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamEventTypePatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     StreamEventTypePatch
     */
    public static StreamEventTypePatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamEventTypePatch.class);
    }

    /**
     * Convert an instance of StreamEventTypePatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
