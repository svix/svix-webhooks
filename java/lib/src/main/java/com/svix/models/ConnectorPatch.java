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

import java.net.URI;
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ConnectorPatch {
    @JsonProperty private MaybeUnset<Set<String>> allowedEventTypes;
    @JsonProperty private String description;
    @JsonProperty private MaybeUnset<Set<String>> featureFlags;
    @JsonProperty private String instructions;
    @JsonProperty private ConnectorKind kind;
    @JsonProperty private MaybeUnset<URI> logo;
    @JsonProperty private String name;
    @JsonProperty private String transformation;

    public ConnectorPatch() {}

    public ConnectorPatch allowedEventTypes(Set<String> allowedEventTypes) {
        this.allowedEventTypes = new MaybeUnset<>(allowedEventTypes);
        return this;
    }

    public ConnectorPatch addAllowedEventTypesItem(String allowedEventTypesItem) {
        if (this.allowedEventTypes == null) {
            this.allowedEventTypes = new MaybeUnset<>(new LinkedHashSet<>());
        }
        this.allowedEventTypes.getValue().add(allowedEventTypesItem);

        return this;
    }

    /**
     * Get allowedEventTypes
     *
     * @return allowedEventTypes
     */
    @javax.annotation.Nullable
    public Set<String> getAllowedEventTypes() {
        if (allowedEventTypes == null) {
            return null;
        }
        return allowedEventTypes.getValue();
    }

    public void setAllowedEventTypes(Set<String> allowedEventTypes) {
        this.allowedEventTypes = new MaybeUnset<>(allowedEventTypes);
    }

    public ConnectorPatch description(String description) {
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

    public ConnectorPatch featureFlags(Set<String> featureFlags) {
        this.featureFlags = new MaybeUnset<>(featureFlags);
        return this;
    }

    public ConnectorPatch addFeatureFlagsItem(String featureFlagsItem) {
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

    public ConnectorPatch instructions(String instructions) {
        this.instructions = instructions;
        return this;
    }

    /**
     * Get instructions
     *
     * @return instructions
     */
    @javax.annotation.Nullable
    public String getInstructions() {
        return instructions;
    }

    public void setInstructions(String instructions) {
        this.instructions = instructions;
    }

    public ConnectorPatch kind(ConnectorKind kind) {
        this.kind = kind;
        return this;
    }

    /**
     * Get kind
     *
     * @return kind
     */
    @javax.annotation.Nullable
    public ConnectorKind getKind() {
        return kind;
    }

    public void setKind(ConnectorKind kind) {
        this.kind = kind;
    }

    public ConnectorPatch logo(URI logo) {
        this.logo = new MaybeUnset<>(logo);
        return this;
    }

    /**
     * Get logo
     *
     * @return logo
     */
    @javax.annotation.Nullable
    public URI getLogo() {
        if (logo == null) {
            return null;
        }
        return logo.getValue();
    }

    public void setLogo(URI logo) {
        this.logo = new MaybeUnset<>(logo);
    }

    public ConnectorPatch name(String name) {
        this.name = name;
        return this;
    }

    /**
     * Get name
     *
     * @return name
     */
    @javax.annotation.Nullable
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public ConnectorPatch transformation(String transformation) {
        this.transformation = transformation;
        return this;
    }

    /**
     * Get transformation
     *
     * @return transformation
     */
    @javax.annotation.Nullable
    public String getTransformation() {
        return transformation;
    }

    public void setTransformation(String transformation) {
        this.transformation = transformation;
    }

    /**
     * Create an instance of ConnectorPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ConnectorPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to ConnectorPatch
     */
    public static ConnectorPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ConnectorPatch.class);
    }

    /**
     * Convert an instance of ConnectorPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
