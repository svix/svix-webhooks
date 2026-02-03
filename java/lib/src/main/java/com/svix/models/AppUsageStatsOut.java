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
public class AppUsageStatsOut {
    @JsonProperty private String id;
    @JsonProperty private BackgroundTaskStatus status;
    @JsonProperty private BackgroundTaskType task;
    @JsonProperty private Set<String> unresolvedAppIds;
    @JsonProperty private OffsetDateTime updatedAt;

    public AppUsageStatsOut() {}

    public AppUsageStatsOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The QueueBackgroundTask's ID.
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

    public AppUsageStatsOut status(BackgroundTaskStatus status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nonnull
    public BackgroundTaskStatus getStatus() {
        return status;
    }

    public void setStatus(BackgroundTaskStatus status) {
        this.status = status;
    }

    public AppUsageStatsOut task(BackgroundTaskType task) {
        this.task = task;
        return this;
    }

    /**
     * Get task
     *
     * @return task
     */
    @javax.annotation.Nonnull
    public BackgroundTaskType getTask() {
        return task;
    }

    public void setTask(BackgroundTaskType task) {
        this.task = task;
    }

    public AppUsageStatsOut unresolvedAppIds(Set<String> unresolvedAppIds) {
        this.unresolvedAppIds = unresolvedAppIds;
        return this;
    }

    public AppUsageStatsOut addUnresolvedAppIdsItem(String unresolvedAppIdsItem) {
        if (this.unresolvedAppIds == null) {
            this.unresolvedAppIds = new LinkedHashSet<>();
        }
        this.unresolvedAppIds.add(unresolvedAppIdsItem);

        return this;
    }

    /**
     * Any app IDs or UIDs received in the request that weren't found.
     *
     * <p>Stats will be produced for all the others.
     *
     * @return unresolvedAppIds
     */
    @javax.annotation.Nonnull
    public Set<String> getUnresolvedAppIds() {
        return unresolvedAppIds;
    }

    public void setUnresolvedAppIds(Set<String> unresolvedAppIds) {
        this.unresolvedAppIds = unresolvedAppIds;
    }

    public AppUsageStatsOut updatedAt(OffsetDateTime updatedAt) {
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

    /**
     * Create an instance of AppUsageStatsOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AppUsageStatsOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AppUsageStatsOut
     */
    public static AppUsageStatsOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AppUsageStatsOut.class);
    }

    /**
     * Convert an instance of AppUsageStatsOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
