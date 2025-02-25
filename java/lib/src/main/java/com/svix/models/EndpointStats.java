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
public class EndpointStats {
    @JsonProperty private Long fail;
    @JsonProperty private Long pending;
    @JsonProperty private Long sending;
    @JsonProperty private Long success;

    public EndpointStats() {}

    public EndpointStats fail(Long fail) {
        this.fail = fail;
        return this;
    }

    /**
     * Get fail
     *
     * @return fail
     */
    @javax.annotation.Nonnull
    public Long getFail() {
        return fail;
    }

    public void setFail(Long fail) {
        this.fail = fail;
    }

    public EndpointStats pending(Long pending) {
        this.pending = pending;
        return this;
    }

    /**
     * Get pending
     *
     * @return pending
     */
    @javax.annotation.Nonnull
    public Long getPending() {
        return pending;
    }

    public void setPending(Long pending) {
        this.pending = pending;
    }

    public EndpointStats sending(Long sending) {
        this.sending = sending;
        return this;
    }

    /**
     * Get sending
     *
     * @return sending
     */
    @javax.annotation.Nonnull
    public Long getSending() {
        return sending;
    }

    public void setSending(Long sending) {
        this.sending = sending;
    }

    public EndpointStats success(Long success) {
        this.success = success;
        return this;
    }

    /**
     * Get success
     *
     * @return success
     */
    @javax.annotation.Nonnull
    public Long getSuccess() {
        return success;
    }

    public void setSuccess(Long success) {
        this.success = success;
    }

    /**
     * Create an instance of EndpointStats given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointStats
     * @throws JsonProcessingException if the JSON string is invalid with respect to EndpointStats
     */
    public static EndpointStats fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointStats.class);
    }

    /**
     * Convert an instance of EndpointStats to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
