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
public class BigQueryConfigPatch {
    @JsonProperty private String projectId;
    @JsonProperty private String datasetId;
    @JsonProperty private String tableId;
    @JsonProperty private String credentials;

    public BigQueryConfigPatch() {}

    public BigQueryConfigPatch projectId(String projectId) {
        this.projectId = projectId;
        return this;
    }

    /**
     * Get projectId
     *
     * @return projectId
     */
    @javax.annotation.Nullable
    public String getProjectId() {
        return projectId;
    }

    public void setProjectId(String projectId) {
        this.projectId = projectId;
    }

    public BigQueryConfigPatch datasetId(String datasetId) {
        this.datasetId = datasetId;
        return this;
    }

    /**
     * Get datasetId
     *
     * @return datasetId
     */
    @javax.annotation.Nullable
    public String getDatasetId() {
        return datasetId;
    }

    public void setDatasetId(String datasetId) {
        this.datasetId = datasetId;
    }

    public BigQueryConfigPatch tableId(String tableId) {
        this.tableId = tableId;
        return this;
    }

    /**
     * Get tableId
     *
     * @return tableId
     */
    @javax.annotation.Nullable
    public String getTableId() {
        return tableId;
    }

    public void setTableId(String tableId) {
        this.tableId = tableId;
    }

    public BigQueryConfigPatch credentials(String credentials) {
        this.credentials = credentials;
        return this;
    }

    /**
     * Get credentials
     *
     * @return credentials
     */
    @javax.annotation.Nullable
    public String getCredentials() {
        return credentials;
    }

    public void setCredentials(String credentials) {
        this.credentials = credentials;
    }

    /**
     * Create an instance of BigQueryConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BigQueryConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     BigQueryConfigPatch
     */
    public static BigQueryConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BigQueryConfigPatch.class);
    }

    /**
     * Convert an instance of BigQueryConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
