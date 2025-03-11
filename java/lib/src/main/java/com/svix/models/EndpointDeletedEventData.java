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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointDeletedEventData {
    @JsonProperty private String appId;
    @JsonProperty private String appUid;
    @JsonProperty private String endpointId;
    @JsonProperty private String endpointUid;

    public EndpointDeletedEventData() {}

    public EndpointDeletedEventData appId(String appId) {
        this.appId = appId;
        return this;
    }

    /**
     * The Application's ID.
     *
     * @return appId
     */
    @javax.annotation.Nonnull
    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }

    public EndpointDeletedEventData appUid(String appUid) {
        this.appUid = appUid;
        return this;
    }

    /**
     * The Application's UID.
     *
     * @return appUid
     */
    @javax.annotation.Nullable
    public String getAppUid() {
        return appUid;
    }

    public void setAppUid(String appUid) {
        this.appUid = appUid;
    }

    public EndpointDeletedEventData endpointId(String endpointId) {
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

    public EndpointDeletedEventData endpointUid(String endpointUid) {
        this.endpointUid = endpointUid;
        return this;
    }

    /**
     * The Endpoint's UID.
     *
     * @return endpointUid
     */
    @javax.annotation.Nullable
    public String getEndpointUid() {
        return endpointUid;
    }

    public void setEndpointUid(String endpointUid) {
        this.endpointUid = endpointUid;
    }

    /**
     * Create an instance of EndpointDeletedEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointDeletedEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointDeletedEventData
     */
    public static EndpointDeletedEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointDeletedEventData.class);
    }

    /**
     * Convert an instance of EndpointDeletedEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
