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
public class PollerV2PollOut {
    @JsonProperty private List<PollerV2MessageOut> data;
    @JsonProperty private Boolean done;

    public PollerV2PollOut() {}

    public PollerV2PollOut data(List<PollerV2MessageOut> data) {
        this.data = data;
        return this;
    }

    public PollerV2PollOut addDataItem(PollerV2MessageOut dataItem) {
        if (this.data == null) {
            this.data = new ArrayList<>();
        }
        this.data.add(dataItem);

        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public List<PollerV2MessageOut> getData() {
        return data;
    }

    public void setData(List<PollerV2MessageOut> data) {
        this.data = data;
    }

    public PollerV2PollOut done(Boolean done) {
        this.done = done;
        return this;
    }

    /**
     * Get done
     *
     * @return done
     */
    @javax.annotation.Nonnull
    public Boolean getDone() {
        return done;
    }

    public void setDone(Boolean done) {
        this.done = done;
    }

    /**
     * Create an instance of PollerV2PollOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PollerV2PollOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to PollerV2PollOut
     */
    public static PollerV2PollOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PollerV2PollOut.class);
    }

    /**
     * Convert an instance of PollerV2PollOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
