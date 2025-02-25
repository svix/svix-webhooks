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
public class ListResponseIntegrationOut {
    @JsonProperty private List<IntegrationOut> data;
    @JsonProperty private Boolean done;
    @JsonProperty private String iterator;
    @JsonProperty private String prevIterator;

    public ListResponseIntegrationOut() {}

    public ListResponseIntegrationOut data(List<IntegrationOut> data) {
        this.data = data;
        return this;
    }

    public ListResponseIntegrationOut addDataItem(IntegrationOut dataItem) {
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
    public List<IntegrationOut> getData() {
        return data;
    }

    public void setData(List<IntegrationOut> data) {
        this.data = data;
    }

    public ListResponseIntegrationOut done(Boolean done) {
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

    public ListResponseIntegrationOut iterator(String iterator) {
        this.iterator = iterator;
        return this;
    }

    /**
     * Get iterator
     *
     * @return iterator
     */
    @javax.annotation.Nullable
    public String getIterator() {
        return iterator;
    }

    public void setIterator(String iterator) {
        this.iterator = iterator;
    }

    public ListResponseIntegrationOut prevIterator(String prevIterator) {
        this.prevIterator = prevIterator;
        return this;
    }

    /**
     * Get prevIterator
     *
     * @return prevIterator
     */
    @javax.annotation.Nullable
    public String getPrevIterator() {
        return prevIterator;
    }

    public void setPrevIterator(String prevIterator) {
        this.prevIterator = prevIterator;
    }

    /**
     * Create an instance of ListResponseIntegrationOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ListResponseIntegrationOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ListResponseIntegrationOut
     */
    public static ListResponseIntegrationOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ListResponseIntegrationOut.class);
    }

    /**
     * Convert an instance of ListResponseIntegrationOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
