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
public class AutoConfigSubscriptionOut {
    @JsonProperty private OffsetDateTime createdAt;
    @JsonProperty private String tokenCensored;
    @JsonProperty private String id;
    @JsonProperty private String endpId;
    @JsonProperty private String destId;
    @JsonProperty private Status status;

    public AutoConfigSubscriptionOut() {}

    public AutoConfigSubscriptionOut createdAt(OffsetDateTime createdAt) {
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

    public AutoConfigSubscriptionOut tokenCensored(String tokenCensored) {
        this.tokenCensored = tokenCensored;
        return this;
    }

    /**
     * Get tokenCensored
     *
     * @return tokenCensored
     */
    @javax.annotation.Nonnull
    public String getTokenCensored() {
        return tokenCensored;
    }

    public void setTokenCensored(String tokenCensored) {
        this.tokenCensored = tokenCensored;
    }

    public AutoConfigSubscriptionOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The AutoConfigSubscription's ID.
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

    public AutoConfigSubscriptionOut endpId(String endpId) {
        this.endpId = endpId;
        return this;
    }

    /**
     * The Endpoint's ID.
     *
     * @return endpId
     */
    @javax.annotation.Nullable
    public String getEndpId() {
        return endpId;
    }

    public void setEndpId(String endpId) {
        this.endpId = endpId;
    }

    public AutoConfigSubscriptionOut destId(String destId) {
        this.destId = destId;
        return this;
    }

    /**
     * The StreamSink's ID.
     *
     * @return destId
     */
    @javax.annotation.Nullable
    public String getDestId() {
        return destId;
    }

    public void setDestId(String destId) {
        this.destId = destId;
    }

    public AutoConfigSubscriptionOut status(Status status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nonnull
    public Status getStatus() {
        return status;
    }

    public void setStatus(Status status) {
        this.status = status;
    }

    /**
     * Create an instance of AutoConfigSubscriptionOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AutoConfigSubscriptionOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AutoConfigSubscriptionOut
     */
    public static AutoConfigSubscriptionOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AutoConfigSubscriptionOut.class);
    }

    /**
     * Convert an instance of AutoConfigSubscriptionOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
