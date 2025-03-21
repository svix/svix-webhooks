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
public class PollingEndpointOut {
    @JsonProperty private List<PollingEndpointMessageOut> data;
    @JsonProperty private Boolean done;
    @JsonProperty private String iterator;

    public PollingEndpointOut() {}

    public PollingEndpointOut data(List<PollingEndpointMessageOut> data) {
        this.data = data;
        return this;
    }

    public PollingEndpointOut addDataItem(PollingEndpointMessageOut dataItem) {
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
    public List<PollingEndpointMessageOut> getData() {
        return data;
    }

    public void setData(List<PollingEndpointMessageOut> data) {
        this.data = data;
    }

    public PollingEndpointOut done(Boolean done) {
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

    public PollingEndpointOut iterator(String iterator) {
        this.iterator = iterator;
        return this;
    }

    /**
     * Get iterator
     *
     * @return iterator
     */
    @javax.annotation.Nonnull
    public String getIterator() {
        return iterator;
    }

    public void setIterator(String iterator) {
        this.iterator = iterator;
    }

    /**
     * Create an instance of PollingEndpointOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PollingEndpointOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PollingEndpointOut
     */
    public static PollingEndpointOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PollingEndpointOut.class);
    }

    /**
     * Convert an instance of PollingEndpointOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
