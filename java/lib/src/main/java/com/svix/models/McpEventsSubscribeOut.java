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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class McpEventsSubscribeOut {
    @JsonProperty private String id;
    @JsonProperty private String endpointId;
    @JsonProperty private OffsetDateTime refreshBefore;
    @JsonProperty private String cursor;
    @JsonProperty private Boolean truncated;

    public McpEventsSubscribeOut() {}

    public McpEventsSubscribeOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The subscription's ID, which is also the UID of its endpoint.
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

    public McpEventsSubscribeOut endpointId(String endpointId) {
        this.endpointId = endpointId;
        return this;
    }

    /**
     * The Endpoint's ID.
     *
     * @return endpointId
     */
    @javax.annotation.Nonnull
    public String getEndpointId() {
        return endpointId;
    }

    public void setEndpointId(String endpointId) {
        this.endpointId = endpointId;
    }

    public McpEventsSubscribeOut refreshBefore(OffsetDateTime refreshBefore) {
        this.refreshBefore = refreshBefore;
        return this;
    }

    /**
     * When the subscription expires, or `null` if it never does.
     *
     * @return refreshBefore
     */
    @javax.annotation.Nullable
    public OffsetDateTime getRefreshBefore() {
        return refreshBefore;
    }

    public void setRefreshBefore(OffsetDateTime refreshBefore) {
        this.refreshBefore = refreshBefore;
    }

    public McpEventsSubscribeOut cursor(String cursor) {
        this.cursor = cursor;
        return this;
    }

    /**
     * Always `null`, since events can't be replayed.
     *
     * @return cursor
     */
    @javax.annotation.Nullable
    public String getCursor() {
        return cursor;
    }

    public void setCursor(String cursor) {
        this.cursor = cursor;
    }

    public McpEventsSubscribeOut truncated(Boolean truncated) {
        this.truncated = truncated;
        return this;
    }

    /**
     * Always `false`, since events can't be replayed.
     *
     * @return truncated
     */
    @javax.annotation.Nonnull
    public Boolean getTruncated() {
        return truncated;
    }

    public void setTruncated(Boolean truncated) {
        this.truncated = truncated;
    }

    /**
     * Create an instance of McpEventsSubscribeOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of McpEventsSubscribeOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     McpEventsSubscribeOut
     */
    public static McpEventsSubscribeOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, McpEventsSubscribeOut.class);
    }

    /**
     * Convert an instance of McpEventsSubscribeOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
