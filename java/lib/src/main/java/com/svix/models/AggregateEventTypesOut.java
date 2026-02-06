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
public class AggregateEventTypesOut {
    @JsonProperty private String id;
    @JsonProperty private BackgroundTaskStatus status;
    @JsonProperty private BackgroundTaskType task;
    @JsonProperty private OffsetDateTime updatedAt;

    public AggregateEventTypesOut() {}

    public AggregateEventTypesOut id(String id) {
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

    public AggregateEventTypesOut status(BackgroundTaskStatus status) {
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

    public AggregateEventTypesOut task(BackgroundTaskType task) {
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

    public AggregateEventTypesOut updatedAt(OffsetDateTime updatedAt) {
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
     * Create an instance of AggregateEventTypesOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AggregateEventTypesOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AggregateEventTypesOut
     */
    public static AggregateEventTypesOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AggregateEventTypesOut.class);
    }

    /**
     * Convert an instance of AggregateEventTypesOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
