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
import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EnvironmentOut {
    @JsonProperty private List<ConnectorOut> connectors;
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private List<EventTypeOut> eventTypes;
    @JsonProperty private Object settings;
    @JsonProperty private Long version;

    public EnvironmentOut() {}

    public EnvironmentOut connectors(List<ConnectorOut> connectors) {
        this.connectors = connectors;
        return this;
    }

    public EnvironmentOut addConnectorsItem(ConnectorOut connectorsItem) {
        if (this.connectors == null) {
            this.connectors = new ArrayList<>();
        }
        this.connectors.add(connectorsItem);

        return this;
    }

    /**
     * Get connectors
     *
     * @return connectors
     */
    @javax.annotation.Nonnull
    public List<ConnectorOut> getConnectors() {
        return connectors;
    }

    public void setConnectors(List<ConnectorOut> connectors) {
        this.connectors = connectors;
    }

    public EnvironmentOut createdAt(OffsetDateTime createdAt) {
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

    public EnvironmentOut eventTypes(List<EventTypeOut> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public EnvironmentOut addEventTypesItem(EventTypeOut eventTypesItem) {
        if (this.eventTypes == null) {
            this.eventTypes = new ArrayList<>();
        }
        this.eventTypes.add(eventTypesItem);

        return this;
    }

    /**
     * Get eventTypes
     *
     * @return eventTypes
     */
    @javax.annotation.Nonnull
    public List<EventTypeOut> getEventTypes() {
        return eventTypes;
    }

    public void setEventTypes(List<EventTypeOut> eventTypes) {
        this.eventTypes = eventTypes;
    }

    public EnvironmentOut settings(Object settings) {
        this.settings = settings;
        return this;
    }

    /**
     * Get settings
     *
     * @return settings
     */
    @javax.annotation.Nullable
    public Object getSettings() {
        return settings;
    }

    public void setSettings(Object settings) {
        this.settings = settings;
    }

    public EnvironmentOut version(Long version) {
        this.version = version;
        return this;
    }

    /**
     * Get version
     *
     * @return version
     */
    @javax.annotation.Nullable
    public Long getVersion() {
        return version;
    }

    public void setVersion(Long version) {
        this.version = version;
    }

    /**
     * Create an instance of EnvironmentOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EnvironmentOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to EnvironmentOut
     */
    public static EnvironmentOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EnvironmentOut.class);
    }

    /**
     * Convert an instance of EnvironmentOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
