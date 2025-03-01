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
public class ExpungeAllContentsOut {
    @JsonProperty private String id;
    @JsonProperty private BackgroundTaskStatus status;
    @JsonProperty private BackgroundTaskType task;

    public ExpungeAllContentsOut() {}

    public ExpungeAllContentsOut id(String id) {
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

    public ExpungeAllContentsOut status(BackgroundTaskStatus status) {
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

    public ExpungeAllContentsOut task(BackgroundTaskType task) {
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

    /**
     * Create an instance of ExpungeAllContentsOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ExpungeAllContentsOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ExpungeAllContentsOut
     */
    public static ExpungeAllContentsOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ExpungeAllContentsOut.class);
    }

    /**
     * Convert an instance of ExpungeAllContentsOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
