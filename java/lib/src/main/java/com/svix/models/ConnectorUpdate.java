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

import java.net.URI;
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ConnectorUpdate {
    @JsonProperty private Set<String> allowedEventTypes;
    @JsonProperty private String description;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String instructions;
    @JsonProperty private ConnectorKind kind;
    @JsonProperty private URI logo;
    @JsonProperty private String name;
    @JsonProperty private String transformation;

    public ConnectorUpdate() {}

    public ConnectorUpdate allowedEventTypes(Set<String> allowedEventTypes) {
        this.allowedEventTypes = allowedEventTypes;
        return this;
    }

    public ConnectorUpdate addAllowedEventTypesItem(String allowedEventTypesItem) {
        if (this.allowedEventTypes == null) {
            this.allowedEventTypes = new LinkedHashSet<>();
        }
        this.allowedEventTypes.add(allowedEventTypesItem);

        return this;
    }

    /**
     * Get allowedEventTypes
     *
     * @return allowedEventTypes
     */
    @javax.annotation.Nullable
    public Set<String> getAllowedEventTypes() {
        return allowedEventTypes;
    }

    public void setAllowedEventTypes(Set<String> allowedEventTypes) {
        this.allowedEventTypes = allowedEventTypes;
    }

    public ConnectorUpdate description(String description) {
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

    public ConnectorUpdate featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public ConnectorUpdate addFeatureFlagsItem(String featureFlagsItem) {
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

    public ConnectorUpdate instructions(String instructions) {
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

    public ConnectorUpdate kind(ConnectorKind kind) {
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

    public ConnectorUpdate logo(URI logo) {
        this.logo = logo;
        return this;
    }

    /**
     * Get logo
     *
     * @return logo
     */
    @javax.annotation.Nullable
    public URI getLogo() {
        return logo;
    }

    public void setLogo(URI logo) {
        this.logo = logo;
    }

    public ConnectorUpdate name(String name) {
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

    public ConnectorUpdate transformation(String transformation) {
        this.transformation = transformation;
        return this;
    }

    /**
     * Get transformation
     *
     * @return transformation
     */
    @javax.annotation.Nonnull
    public String getTransformation() {
        return transformation;
    }

    public void setTransformation(String transformation) {
        this.transformation = transformation;
    }

    /**
     * Create an instance of ConnectorUpdate given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ConnectorUpdate
     * @throws JsonProcessingException if the JSON string is invalid with respect to ConnectorUpdate
     */
    public static ConnectorUpdate fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ConnectorUpdate.class);
    }

    /**
     * Convert an instance of ConnectorUpdate to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
