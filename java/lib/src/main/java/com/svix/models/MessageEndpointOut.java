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
public class MessageEndpointOut {
    @JsonProperty private Set<String> channels;
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private String description;
    @JsonProperty private Boolean disabled;
    @JsonProperty private Set<String> filterTypes;
    @JsonProperty private String id;
    @JsonProperty private OffsetDateTime nextAttempt;
    @JsonProperty private Long rateLimit;
    @JsonProperty private MessageStatus status;
    @JsonProperty private MessageStatusText statusText;
    @JsonProperty private String uid;
    @JsonProperty private OffsetDateTime updatedAt;
    @JsonProperty private URI url;
    @JsonProperty private Integer version;

    public MessageEndpointOut() {}

    public MessageEndpointOut channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public MessageEndpointOut addChannelsItem(String channelsItem) {
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

    public MessageEndpointOut createdAt(OffsetDateTime createdAt) {
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

    public MessageEndpointOut description(String description) {
        this.description = description;
        return this;
    }

    /**
     * An example endpoint name.
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

    public MessageEndpointOut disabled(Boolean disabled) {
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

    public MessageEndpointOut filterTypes(Set<String> filterTypes) {
        this.filterTypes = filterTypes;
        return this;
    }

    public MessageEndpointOut addFilterTypesItem(String filterTypesItem) {
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

    public MessageEndpointOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The Endpoint's ID.
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

    public MessageEndpointOut nextAttempt(OffsetDateTime nextAttempt) {
        this.nextAttempt = nextAttempt;
        return this;
    }

    /**
     * Get nextAttempt
     *
     * @return nextAttempt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getNextAttempt() {
        return nextAttempt;
    }

    public void setNextAttempt(OffsetDateTime nextAttempt) {
        this.nextAttempt = nextAttempt;
    }

    public MessageEndpointOut rateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
        return this;
    }

    /**
     * Get rateLimit
     *
     * @return rateLimit
     */
    @javax.annotation.Nullable
    public Long getRateLimit() {
        return rateLimit;
    }

    public void setRateLimit(Long rateLimit) {
        this.rateLimit = rateLimit;
    }

    public MessageEndpointOut status(MessageStatus status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nonnull
    public MessageStatus getStatus() {
        return status;
    }

    public void setStatus(MessageStatus status) {
        this.status = status;
    }

    public MessageEndpointOut statusText(MessageStatusText statusText) {
        this.statusText = statusText;
        return this;
    }

    /**
     * Get statusText
     *
     * @return statusText
     */
    @javax.annotation.Nonnull
    public MessageStatusText getStatusText() {
        return statusText;
    }

    public void setStatusText(MessageStatusText statusText) {
        this.statusText = statusText;
    }

    public MessageEndpointOut uid(String uid) {
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

    public MessageEndpointOut updatedAt(OffsetDateTime updatedAt) {
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

    public MessageEndpointOut url(URI url) {
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
    public MessageEndpointOut version(Integer version) {
        this.version = version;
        return this;
    }

    /**
     * Get version
     *
     * @return version
     */
    @javax.annotation.Nonnull
    @Deprecated
    public Integer getVersion() {
        return version;
    }

    @Deprecated
    public void setVersion(Integer version) {
        this.version = version;
    }

    /**
     * Create an instance of MessageEndpointOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageEndpointOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageEndpointOut
     */
    public static MessageEndpointOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageEndpointOut.class);
    }

    /**
     * Convert an instance of MessageEndpointOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
