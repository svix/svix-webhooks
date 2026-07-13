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
public class BigQueryConfig {
    @JsonProperty private String projectId;
    @JsonProperty private String datasetId;
    @JsonProperty private String tableId;
    @JsonProperty private String credentials;

    public BigQueryConfig() {}

    public BigQueryConfig projectId(String projectId) {
        this.projectId = projectId;
        return this;
    }

    /**
     * Get projectId
     *
     * @return projectId
     */
    @javax.annotation.Nonnull
    public String getProjectId() {
        return projectId;
    }

    public void setProjectId(String projectId) {
        this.projectId = projectId;
    }

    public BigQueryConfig datasetId(String datasetId) {
        this.datasetId = datasetId;
        return this;
    }

    /**
     * Get datasetId
     *
     * @return datasetId
     */
    @javax.annotation.Nonnull
    public String getDatasetId() {
        return datasetId;
    }

    public void setDatasetId(String datasetId) {
        this.datasetId = datasetId;
    }

    public BigQueryConfig tableId(String tableId) {
        this.tableId = tableId;
        return this;
    }

    /**
     * Get tableId
     *
     * @return tableId
     */
    @javax.annotation.Nonnull
    public String getTableId() {
        return tableId;
    }

    public void setTableId(String tableId) {
        this.tableId = tableId;
    }

    public BigQueryConfig credentials(String credentials) {
        this.credentials = credentials;
        return this;
    }

    /**
     * Google Cloud Credentials JSON Object as a string.
     *
     * @return credentials
     */
    @javax.annotation.Nonnull
    public String getCredentials() {
        return credentials;
    }

    public void setCredentials(String credentials) {
        this.credentials = credentials;
    }

    /**
     * Create an instance of BigQueryConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BigQueryConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to BigQueryConfig
     */
    public static BigQueryConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BigQueryConfig.class);
    }

    /**
     * Convert an instance of BigQueryConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
