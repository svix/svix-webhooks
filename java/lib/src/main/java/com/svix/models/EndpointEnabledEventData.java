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
public class EndpointEnabledEventData {
    @JsonProperty private String appId;
    @JsonProperty private String appUid;
    @JsonProperty private String endpointId;
    @JsonProperty private String endpointUid;

    public EndpointEnabledEventData() {}

    public EndpointEnabledEventData appId(String appId) {
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

    public EndpointEnabledEventData appUid(String appUid) {
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

    public EndpointEnabledEventData endpointId(String endpointId) {
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

    public EndpointEnabledEventData endpointUid(String endpointUid) {
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
     * Create an instance of EndpointEnabledEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointEnabledEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointEnabledEventData
     */
    public static EndpointEnabledEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointEnabledEventData.class);
    }

    /**
     * Convert an instance of EndpointEnabledEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
