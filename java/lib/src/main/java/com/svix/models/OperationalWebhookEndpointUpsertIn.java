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
import java.util.HashMap;
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class OperationalWebhookEndpointUpsertIn {
    @JsonProperty private String description;
    @JsonProperty private Short throttleRate;
    @JsonProperty private String uid;
    @JsonProperty private URI url;
    @JsonProperty private Boolean disabled;
    @JsonProperty private Set<String> eventTypesIds;
    @JsonProperty private Map<String, String> metadata;

    public OperationalWebhookEndpointUpsertIn() {}

    public OperationalWebhookEndpointUpsertIn description(String description) {
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

    public OperationalWebhookEndpointUpsertIn throttleRate(Short throttleRate) {
        this.throttleRate = throttleRate;
        return this;
    }

    /**
     * Maximum messages per second to send to this endpoint.
     *
     * <p>Outgoing messages will be throttled to this rate.
     *
     * @return throttleRate
     */
    @javax.annotation.Nullable
    public Short getThrottleRate() {
        return throttleRate;
    }

    public void setThrottleRate(Short throttleRate) {
        this.throttleRate = throttleRate;
    }

    public OperationalWebhookEndpointUpsertIn uid(String uid) {
        this.uid = uid;
        return this;
    }

    /**
     * Optional unique identifier for the endpoint.
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

    public OperationalWebhookEndpointUpsertIn url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public OperationalWebhookEndpointUpsertIn disabled(Boolean disabled) {
        this.disabled = disabled;
        return this;
    }

    /**
     * Get disabled
     *
     * @return disabled
     */
    @javax.annotation.Nullable
    public Boolean getDisabled() {
        return disabled;
    }

    public void setDisabled(Boolean disabled) {
        this.disabled = disabled;
    }

    public OperationalWebhookEndpointUpsertIn eventTypesIds(Set<String> eventTypesIds) {
        this.eventTypesIds = eventTypesIds;
        return this;
    }

    public OperationalWebhookEndpointUpsertIn addEventTypesIdsItem(String eventTypesIdsItem) {
        if (this.eventTypesIds == null) {
            this.eventTypesIds = new LinkedHashSet<>();
        }
        this.eventTypesIds.add(eventTypesIdsItem);

        return this;
    }

    /**
     * Get eventTypesIds
     *
     * @return eventTypesIds
     */
    @javax.annotation.Nullable
    public Set<String> getEventTypesIds() {
        return eventTypesIds;
    }

    public void setEventTypesIds(Set<String> eventTypesIds) {
        this.eventTypesIds = eventTypesIds;
    }

    public OperationalWebhookEndpointUpsertIn metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public OperationalWebhookEndpointUpsertIn putMetadataItem(String key, String metadataItem) {
        if (this.metadata == null) {
            this.metadata = new HashMap<>();
        }
        this.metadata.put(key, metadataItem);

        return this;
    }

    /**
     * Get metadata
     *
     * @return metadata
     */
    @javax.annotation.Nullable
    public Map<String, String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String, String> metadata) {
        this.metadata = metadata;
    }

    /**
     * Create an instance of OperationalWebhookEndpointUpsertIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of OperationalWebhookEndpointUpsertIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     OperationalWebhookEndpointUpsertIn
     */
    public static OperationalWebhookEndpointUpsertIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper()
                .readValue(jsonString, OperationalWebhookEndpointUpsertIn.class);
    }

    /**
     * Convert an instance of OperationalWebhookEndpointUpsertIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
