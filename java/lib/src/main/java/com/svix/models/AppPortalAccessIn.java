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
public class AppPortalAccessIn {
    @JsonProperty private ApplicationIn application;
    @JsonProperty private Long expiry;
    @JsonProperty private Set<String> featureFlags;
    @JsonProperty private Boolean readOnly;
    @JsonProperty private String sessionId;

    public AppPortalAccessIn() {}

    public AppPortalAccessIn application(ApplicationIn application) {
        this.application = application;
        return this;
    }

    /**
     * Optionally creates a new application while generating the access link.
     *
     * <p>If the application id or uid that is used in the path already exists, this argument is
     * ignored.
     *
     * @return application
     */
    @javax.annotation.Nullable
    public ApplicationIn getApplication() {
        return application;
    }

    public void setApplication(ApplicationIn application) {
        this.application = application;
    }

    public AppPortalAccessIn expiry(Long expiry) {
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

    public AppPortalAccessIn featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public AppPortalAccessIn addFeatureFlagsItem(String featureFlagsItem) {
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

    public AppPortalAccessIn readOnly(Boolean readOnly) {
        this.readOnly = readOnly;
        return this;
    }

    /**
     * Whether the app portal should be in read-only mode.
     *
     * @return readOnly
     */
    @javax.annotation.Nullable
    public Boolean getReadOnly() {
        return readOnly;
    }

    public void setReadOnly(Boolean readOnly) {
        this.readOnly = readOnly;
    }

    public AppPortalAccessIn sessionId(String sessionId) {
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
     * Create an instance of AppPortalAccessIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AppPortalAccessIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AppPortalAccessIn
     */
    public static AppPortalAccessIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AppPortalAccessIn.class);
    }

    /**
     * Convert an instance of AppPortalAccessIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
