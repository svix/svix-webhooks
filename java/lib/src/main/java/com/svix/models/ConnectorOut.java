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
import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ConnectorOut {
    @JsonProperty private Set<String> allowedEventTypes;
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private String description;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String id;
    @JsonProperty private String instructions;
    @JsonProperty private ConnectorKind kind;
    @JsonProperty private URI logo;
    @JsonProperty private String name;
    @JsonProperty private String orgId;
    @JsonProperty private ConnectorProduct productType;
    @JsonProperty private String transformation;
    @JsonProperty private OffsetDateTime transformationUpdatedAt;
    @JsonProperty private String uid;
    @JsonProperty private OffsetDateTime updatedAt;

    public ConnectorOut() {}

    public ConnectorOut allowedEventTypes(Set<String> allowedEventTypes) {
        this.allowedEventTypes = allowedEventTypes;
        return this;
    }

    public ConnectorOut addAllowedEventTypesItem(String allowedEventTypesItem) {
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

    public ConnectorOut createdAt(OffsetDateTime createdAt) {
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

    public ConnectorOut description(String description) {
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

    public ConnectorOut featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public ConnectorOut addFeatureFlagsItem(String featureFlagsItem) {
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

    public ConnectorOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The Connector's ID.
     *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public ConnectorOut instructions(String instructions) {
        this.instructions = instructions;
        return this;
    }

    /**
     * Get instructions
     *
     * @return instructions
     */
    @javax.annotation.Nonnull
    public String getInstructions() {
        return instructions;
    }

    public void setInstructions(String instructions) {
        this.instructions = instructions;
    }

    public ConnectorOut kind(ConnectorKind kind) {
        this.kind = kind;
        return this;
    }

    /**
     * Get kind
     *
     * @return kind
     */
    @javax.annotation.Nonnull
    public ConnectorKind getKind() {
        return kind;
    }

    public void setKind(ConnectorKind kind) {
        this.kind = kind;
    }

    public ConnectorOut logo(URI logo) {
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

    public ConnectorOut name(String name) {
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

    public ConnectorOut orgId(String orgId) {
        this.orgId = orgId;
        return this;
    }

    /**
     * The Environment's ID.
     *
     * @return orgId
     */
    @javax.annotation.Nonnull
    public String getOrgId() {
        return orgId;
    }

    public void setOrgId(String orgId) {
        this.orgId = orgId;
    }

    public ConnectorOut productType(ConnectorProduct productType) {
        this.productType = productType;
        return this;
    }

    /**
     * Get productType
     *
     * @return productType
     */
    @javax.annotation.Nonnull
    public ConnectorProduct getProductType() {
        return productType;
    }

    public void setProductType(ConnectorProduct productType) {
        this.productType = productType;
    }

    public ConnectorOut transformation(String transformation) {
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

    public ConnectorOut transformationUpdatedAt(OffsetDateTime transformationUpdatedAt) {
        this.transformationUpdatedAt = transformationUpdatedAt;
        return this;
    }

    /**
     * Get transformationUpdatedAt
     *
     * @return transformationUpdatedAt
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getTransformationUpdatedAt() {
        return transformationUpdatedAt;
    }

    public void setTransformationUpdatedAt(OffsetDateTime transformationUpdatedAt) {
        this.transformationUpdatedAt = transformationUpdatedAt;
    }

    public ConnectorOut uid(String uid) {
        this.uid = uid;
        return this;
    }

    /**
     * The Connector's UID.
     *
     * @return uid
     */
    @javax.annotation.Nullable
    public String getUid() {
        return uid;
    }

    public void setUid(String uid) {
        this.uid = uid;
    }

    public ConnectorOut updatedAt(OffsetDateTime updatedAt) {
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
     * Create an instance of ConnectorOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ConnectorOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ConnectorOut
     */
    public static ConnectorOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ConnectorOut.class);
    }

    /**
     * Convert an instance of ConnectorOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
