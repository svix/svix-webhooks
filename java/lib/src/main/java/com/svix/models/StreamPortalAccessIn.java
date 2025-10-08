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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class StreamPortalAccessIn {
    @JsonProperty private Long expiry;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private String sessionId;

    public StreamPortalAccessIn() {}

    public StreamPortalAccessIn expiry(Long expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
     * How long the token will be valid for, in seconds.
     *
     * <p>Valid values are between 1 hour and 7 days. The default is 7 days.
     *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Long getExpiry() {
        return expiry;
    }

    public void setExpiry(Long expiry) {
        this.expiry = expiry;
    }

    public StreamPortalAccessIn featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public StreamPortalAccessIn addFeatureFlagsItem(String featureFlagsItem) {
        if (this.featureFlags == null) {
            this.featureFlags = new LinkedHashSet<>();
        }
        this.featureFlags.add(featureFlagsItem);

        return this;
    }

    /**
     * The set of feature flags the created token will have access to.
     *
     * @return featureFlags
     */
    @javax.annotation.Nullable
    public Set<String> getFeatureFlags() {
        return featureFlags;
    }

    public void setFeatureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
    }

    public StreamPortalAccessIn sessionId(String sessionId) {
        this.sessionId = sessionId;
        return this;
    }

    /**
     * An optional session ID to attach to the token.
     *
     * <p>When expiring tokens with "Expire All", you can include the session ID to only expire
     * tokens that were created with that session ID.
     *
     * @return sessionId
     */
    @javax.annotation.Nullable
    public String getSessionId() {
        return sessionId;
    }

    public void setSessionId(String sessionId) {
        this.sessionId = sessionId;
    }

    /**
     * Create an instance of StreamPortalAccessIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamPortalAccessIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     StreamPortalAccessIn
     */
    public static StreamPortalAccessIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamPortalAccessIn.class);
    }

    /**
     * Convert an instance of StreamPortalAccessIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
