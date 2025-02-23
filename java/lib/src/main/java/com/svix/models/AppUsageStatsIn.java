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
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class AppUsageStatsIn {
    @JsonProperty private Set<String> appIds;
    @JsonProperty private OffsetDateTime since;
    @JsonProperty private OffsetDateTime until;

    public AppUsageStatsIn() {}

    public AppUsageStatsIn appIds(Set<String> appIds) {
        this.appIds = appIds;
        return this;
    }

    public AppUsageStatsIn addAppIdsItem(String appIdsItem) {
        if (this.appIds == null) {
            this.appIds = new LinkedHashSet<>();
        }
        this.appIds.add(appIdsItem);

        return this;
    }

    /**
     * Specific app IDs or UIDs to aggregate stats for.
     *
     * <p>Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
     *
     * @return appIds
     */
    @javax.annotation.Nullable
    public Set<String> getAppIds() {
        return appIds;
    }

    public void setAppIds(Set<String> appIds) {
        this.appIds = appIds;
    }

    public AppUsageStatsIn since(OffsetDateTime since) {
        this.since = since;
        return this;
    }

    /**
     * Get since
     *
     * @return since
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getSince() {
        return since;
    }

    public void setSince(OffsetDateTime since) {
        this.since = since;
    }

    public AppUsageStatsIn until(OffsetDateTime until) {
        this.until = until;
        return this;
    }

    /**
     * Get until
     *
     * @return until
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getUntil() {
        return until;
    }

    public void setUntil(OffsetDateTime until) {
        this.until = until;
    }

    /**
     * Create an instance of AppUsageStatsIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AppUsageStatsIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to AppUsageStatsIn
     */
    public static AppUsageStatsIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AppUsageStatsIn.class);
    }

    /**
     * Convert an instance of AppUsageStatsIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
