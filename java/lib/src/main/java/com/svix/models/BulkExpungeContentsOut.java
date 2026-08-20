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

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class BulkExpungeContentsOut {
    @JsonProperty private Map<String, BulkExpungeStatus> results;

    public BulkExpungeContentsOut() {}

    public BulkExpungeContentsOut results(Map<String, BulkExpungeStatus> results) {
        this.results = results;
        return this;
    }

    public BulkExpungeContentsOut putResultsItem(String key, BulkExpungeStatus resultsItem) {
        if (this.results == null) {
            this.results = new HashMap<>();
        }
        this.results.put(key, resultsItem);

        return this;
    }

    /**
     * Results of expunging (by ID)
     *
     * @return results
     */
    @javax.annotation.Nonnull
    public Map<String, BulkExpungeStatus> getResults() {
        return results;
    }

    public void setResults(Map<String, BulkExpungeStatus> results) {
        this.results = results;
    }

    /**
     * Create an instance of BulkExpungeContentsOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BulkExpungeContentsOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     BulkExpungeContentsOut
     */
    public static BulkExpungeContentsOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BulkExpungeContentsOut.class);
    }

    /**
     * Convert an instance of BulkExpungeContentsOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
