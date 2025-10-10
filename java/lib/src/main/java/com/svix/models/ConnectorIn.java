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
public class ConnectorIn {
    @JsonProperty private String description;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private Set<String> filterTypes;
    @JsonProperty private String instructions;
    @JsonProperty private ConnectorKind kind;
    @JsonProperty private URI logo;
    @JsonProperty private String name;
    @JsonProperty private String transformation;

    public ConnectorIn() {}

    public ConnectorIn description(String description) {
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

    public ConnectorIn featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public ConnectorIn addFeatureFlagsItem(String featureFlagsItem) {
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

    public ConnectorIn filterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
        return this;
    }

    public ConnectorIn addFilterTypesItem(String filterTypesItem) {
        if (this.filterTypes == null) {
            this.filterTypes = new LinkedHashSet<>();
        }
        this.filterTypes.add(filterTypesItem);

        return this;
    }

    /**
     * Get filterTypes
     *
     * @return filterTypes
     */
    @javax.annotation.Nullable
    public Set<String> getFilterTypes() {
        return filterTypes;
    }

    public void setFilterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
    }

    public ConnectorIn instructions(String instructions) {
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

    public ConnectorIn kind(ConnectorKind kind) {
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

    public ConnectorIn logo(URI logo) {
        this.logo = logo;
        return this;
    }

    /**
     * Get logo
     *
     * @return logo
     */
    @javax.annotation.Nonnull
    public URI getLogo() {
        return logo;
    }

    public void setLogo(URI logo) {
        this.logo = logo;
    }

    public ConnectorIn name(String name) {
        this.name = name;
        return this;
    }

    /**
     * Get name
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

    public ConnectorIn transformation(String transformation) {
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
     * Create an instance of ConnectorIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ConnectorIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to ConnectorIn
     */
    public static ConnectorIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ConnectorIn.class);
    }

    /**
     * Convert an instance of ConnectorIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
