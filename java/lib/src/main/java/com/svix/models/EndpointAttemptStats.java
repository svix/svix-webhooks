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
public class EndpointAttemptStats {
    @JsonProperty private Long success;
    @JsonProperty private Long fail;
    @JsonProperty private Long canceled;

    public EndpointAttemptStats() {}

    public EndpointAttemptStats success(Long success) {
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

    public EndpointAttemptStats fail(Long fail) {
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

    public EndpointAttemptStats canceled(Long canceled) {
        this.canceled = canceled;
        return this;
    }

    /**
     * Get canceled
     *
     * @return canceled
     */
    @javax.annotation.Nonnull
    public Long getCanceled() {
        return canceled;
    }

    public void setCanceled(Long canceled) {
        this.canceled = canceled;
    }

    /**
     * Create an instance of EndpointAttemptStats given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointAttemptStats
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointAttemptStats
     */
    public static EndpointAttemptStats fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointAttemptStats.class);
    }

    /**
     * Convert an instance of EndpointAttemptStats to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
