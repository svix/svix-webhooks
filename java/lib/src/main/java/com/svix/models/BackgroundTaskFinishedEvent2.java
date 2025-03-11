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
public class BackgroundTaskFinishedEvent2 {
    @JsonProperty private Object data;
    @JsonProperty private BackgroundTaskStatus status;
    @JsonProperty private BackgroundTaskType task;
    @JsonProperty private String taskId;

    public BackgroundTaskFinishedEvent2() {}

    public BackgroundTaskFinishedEvent2 data(Object data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public Object getData() {
        return data;
    }

    public void setData(Object data) {
        this.data = data;
    }

    public BackgroundTaskFinishedEvent2 status(BackgroundTaskStatus status) {
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

    public BackgroundTaskFinishedEvent2 task(BackgroundTaskType task) {
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

    public BackgroundTaskFinishedEvent2 taskId(String taskId) {
        this.taskId = taskId;
        return this;
    }

    /**
     * The QueueBackgroundTask's ID.
     *
     * @return taskId
     */
    @javax.annotation.Nonnull
    public String getTaskId() {
        return taskId;
    }

    public void setTaskId(String taskId) {
        this.taskId = taskId;
    }

    /**
     * Create an instance of BackgroundTaskFinishedEvent2 given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BackgroundTaskFinishedEvent2
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     BackgroundTaskFinishedEvent2
     */
    public static BackgroundTaskFinishedEvent2 fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BackgroundTaskFinishedEvent2.class);
    }

    /**
     * Convert an instance of BackgroundTaskFinishedEvent2 to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
