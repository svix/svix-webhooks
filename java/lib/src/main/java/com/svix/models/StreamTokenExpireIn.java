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

import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class StreamTokenExpireIn {
    @JsonProperty private Long expiry;
    @JsonProperty private List<String> sessionIds;

    public StreamTokenExpireIn() {}

    public StreamTokenExpireIn expiry(Long expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
     * How many seconds until the old key is expired.
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

    public StreamTokenExpireIn sessionIds(List<String> sessionIds) {
        this.sessionIds = sessionIds;
        return this;
    }

    public StreamTokenExpireIn addSessionIdsItem(String sessionIdsItem) {
        if (this.sessionIds == null) {
            this.sessionIds = new ArrayList<>();
        }
        this.sessionIds.add(sessionIdsItem);

        return this;
    }

    /**
     * An optional list of session ids.
     *
     * <p>If any session ids are specified, only Stream tokens created with that session id will be
     * expired.
     *
     * @return sessionIds
     */
    @javax.annotation.Nullable
    public List<String> getSessionIds() {
        return sessionIds;
    }

    public void setSessionIds(List<String> sessionIds) {
        this.sessionIds = sessionIds;
    }

    /**
     * Create an instance of StreamTokenExpireIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamTokenExpireIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     StreamTokenExpireIn
     */
    public static StreamTokenExpireIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamTokenExpireIn.class);
    }

    /**
     * Convert an instance of StreamTokenExpireIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
