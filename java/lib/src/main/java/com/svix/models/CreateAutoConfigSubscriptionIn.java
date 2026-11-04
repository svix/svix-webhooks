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
public class CreateAutoConfigSubscriptionIn {
    @JsonProperty private Set<String> featureFlags;

    public CreateAutoConfigSubscriptionIn() {}

    public CreateAutoConfigSubscriptionIn featureFlags(Set<String> featureFlags) {
        this.featureFlags = featureFlags;
        return this;
    }

    public CreateAutoConfigSubscriptionIn addFeatureFlagsItem(String featureFlagsItem) {
        if (this.featureFlags == null) {
            this.featureFlags = new LinkedHashSet<>();
        }
        this.featureFlags.add(featureFlagsItem);

        return this;
    }

    /**
     * The set of feature flags the created token will have access to.
     *
     * <p>When omitted or empty, the token inherits the calling token's feature flags. When set,
     * these flags are used instead. An application token may only grant a subset of its own flags.
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

    /**
     * Create an instance of CreateAutoConfigSubscriptionIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of CreateAutoConfigSubscriptionIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     CreateAutoConfigSubscriptionIn
     */
    public static CreateAutoConfigSubscriptionIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, CreateAutoConfigSubscriptionIn.class);
    }

    /**
     * Convert an instance of CreateAutoConfigSubscriptionIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
