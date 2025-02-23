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

import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EnvironmentIn {
    @JsonProperty private List<ConnectorIn> connectors;
    @JsonProperty private List<EventTypeIn> eventTypes;
    @JsonProperty private Object settings;

    public EnvironmentIn() {}

    public EnvironmentIn connectors(List<ConnectorIn> connectors) {
        this.connectors = connectors;
        return this;
    }

    public EnvironmentIn addConnectorsItem(ConnectorIn connectorsItem) {
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
    @javax.annotation.Nullable
    public List<ConnectorIn> getConnectors() {
        return connectors;
    }

    public void setConnectors(List<ConnectorIn> connectors) {
        this.connectors = connectors;
    }

    public EnvironmentIn eventTypes(List<EventTypeIn> eventTypes) {
        this.eventTypes = eventTypes;
        return this;
    }

    public EnvironmentIn addEventTypesItem(EventTypeIn eventTypesItem) {
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
    @javax.annotation.Nullable
    public List<EventTypeIn> getEventTypes() {
        return eventTypes;
    }

    public void setEventTypes(List<EventTypeIn> eventTypes) {
        this.eventTypes = eventTypes;
    }

    public EnvironmentIn settings(Object settings) {
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

    /**
     * Create an instance of EnvironmentIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EnvironmentIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to EnvironmentIn
     */
    public static EnvironmentIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EnvironmentIn.class);
    }

    /**
     * Convert an instance of EnvironmentIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
