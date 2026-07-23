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
import java.util.HashMap;
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointPatch {
    @JsonProperty private String description;
    @JsonProperty private MaybeUnset<Long> throttleRate;
    @JsonProperty private MaybeUnset<String> uid;
    @JsonProperty private URI url;
    @JsonProperty private Boolean disabled;
    @JsonProperty private MaybeUnset<Set<String>> filterTypes;
    @JsonProperty private MaybeUnset<Set<String>> channels;
    @JsonProperty private Map<String, String> metadata;

    public EndpointPatch() {}

    public EndpointPatch description(String description) {
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

    public EndpointPatch throttleRate(Long throttleRate) {
        this.throttleRate = new MaybeUnset<>(throttleRate);
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
        if (throttleRate == null) {
            return null;
        }
        return throttleRate.getValue();
    }

    public void setThrottleRate(Long throttleRate) {
        this.throttleRate = new MaybeUnset<>(throttleRate);
    }

    public EndpointPatch uid(String uid) {
        this.uid = new MaybeUnset<>(uid);
        return this;
    }

    /**
     * The Endpoint's UID.
     *
     * @return uid
     */
    @javax.annotation.Nullable
    public String getUid() {
        if (uid == null) {
            return null;
        }
        return uid.getValue();
    }

    public void setUid(String uid) {
        this.uid = new MaybeUnset<>(uid);
    }

    public EndpointPatch url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nullable
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public EndpointPatch disabled(Boolean disabled) {
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

    public EndpointPatch filterTypes(Set<String> filterTypes) {
        this.filterTypes = new MaybeUnset<>(filterTypes);
        return this;
    }

    public EndpointPatch addFilterTypesItem(String filterTypesItem) {
        if (this.filterTypes == null) {
            this.filterTypes = new MaybeUnset<>(new LinkedHashSet<>());
        }
        this.filterTypes.getValue().add(filterTypesItem);

        return this;
    }

    /**
     * Get filterTypes
     *
     * @return filterTypes
     */
    @javax.annotation.Nullable
    public Set<String> getFilterTypes() {
        if (filterTypes == null) {
            return null;
        }
        return filterTypes.getValue();
    }

    public void setFilterTypes(Set<String> filterTypes) {
        this.filterTypes = new MaybeUnset<>(filterTypes);
    }

    public EndpointPatch channels(Set<String> channels) {
        this.channels = new MaybeUnset<>(channels);
        return this;
    }

    public EndpointPatch addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new MaybeUnset<>(new LinkedHashSet<>());
        }
        this.channels.getValue().add(channelsItem);

        return this;
    }

    /**
     * Get channels
     *
     * @return channels
     */
    @javax.annotation.Nullable
    public Set<String> getChannels() {
        if (channels == null) {
            return null;
        }
        return channels.getValue();
    }

    public void setChannels(Set<String> channels) {
        this.channels = new MaybeUnset<>(channels);
    }

    public EndpointPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public EndpointPatch putMetadataItem(String key, String metadataItem) {
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
     * Create an instance of EndpointPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to EndpointPatch
     */
    public static EndpointPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointPatch.class);
    }

    /**
     * Convert an instance of EndpointPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
