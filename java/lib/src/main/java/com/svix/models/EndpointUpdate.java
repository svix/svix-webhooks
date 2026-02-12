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
public class EndpointUpdate {
    @JsonProperty private Set<String> channels;
    @JsonProperty private String description;
    @JsonProperty private Boolean disabled;
    @JsonProperty private Set<String> filterTypes;
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private Long rateLimit;
    @JsonProperty private Long throttleRate;
    @JsonProperty private String uid;
    @JsonProperty private URI url;
    @JsonProperty private Long version;

    public EndpointUpdate() {}

    public EndpointUpdate channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public EndpointUpdate addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new LinkedHashSet<>();
        }
        this.channels.add(channelsItem);

        return this;
    }

    /**
     * List of message channels this endpoint listens to (omit for all).
     *
     * @return channels
     */
    @javax.annotation.Nullable
    public Set<String> getChannels() {
        return channels;
    }

    public void setChannels(Set<String> channels) {
        this.channels = channels;
    }

    public EndpointUpdate description(String description) {
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

    public EndpointUpdate disabled(Boolean disabled) {
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

    public EndpointUpdate filterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
        return this;
    }

    public EndpointUpdate addFilterTypesItem(String filterTypesItem) {
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

    public EndpointUpdate metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public EndpointUpdate putMetadataItem(String key, String metadataItem) {
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

    @Deprecated
    public EndpointUpdate rateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
        return this;
    }

    /**
     * Deprecated, use `throttleRate` instead.
     *
     * @return rateLimit
     */
    @javax.annotation.Nullable
    @Deprecated
    public Long getRateLimit() {
        return rateLimit;
    }

    @Deprecated
    public void setRateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
    }

    public EndpointUpdate throttleRate(Long throttleRate) {
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
    public Long getThrottleRate() {
        return throttleRate;
    }

    public void setThrottleRate(Long throttleRate) {
        this.throttleRate = throttleRate;
    }

    public EndpointUpdate uid(String uid) {
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

    public EndpointUpdate url(URI url) {
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

    @Deprecated
    public EndpointUpdate version(Long version) {
        this.version = version;
        return this;
    }

    /**
     * Get version
     *
     * @return version
     */
    @javax.annotation.Nullable
    @Deprecated
    public Long getVersion() {
        return version;
    }

    @Deprecated
    public void setVersion(Long version) {
        this.version = version;
    }

    /**
     * Create an instance of EndpointUpdate given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointUpdate
     * @throws JsonProcessingException if the JSON string is invalid with respect to EndpointUpdate
     */
    public static EndpointUpdate fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointUpdate.class);
    }

    /**
     * Convert an instance of EndpointUpdate to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
