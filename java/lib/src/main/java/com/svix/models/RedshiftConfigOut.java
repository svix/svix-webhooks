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
public class RedshiftConfigOut {
    @JsonProperty private String accessKeyId;
    @JsonProperty private String region;
    @JsonProperty private String clusterIdentifier;
    @JsonProperty private String dbUser;
    @JsonProperty private String workgroupName;
    @JsonProperty private String dbName;
    @JsonProperty private String schemaName;
    @JsonProperty private String tableName;

    public RedshiftConfigOut() {}

    public RedshiftConfigOut accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Get accessKeyId
     *
     * @return accessKeyId
     */
    @javax.annotation.Nonnull
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public RedshiftConfigOut region(String region) {
        this.region = region;
        return this;
    }

    /**
     * Get region
     *
     * @return region
     */
    @javax.annotation.Nonnull
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    public RedshiftConfigOut clusterIdentifier(String clusterIdentifier) {
        this.clusterIdentifier = clusterIdentifier;
        return this;
    }

    /**
     * Get clusterIdentifier
     *
     * @return clusterIdentifier
     */
    @javax.annotation.Nullable
    public String getClusterIdentifier() {
        return clusterIdentifier;
    }

    public void setClusterIdentifier(String clusterIdentifier) {
        this.clusterIdentifier = clusterIdentifier;
    }

    public RedshiftConfigOut dbUser(String dbUser) {
        this.dbUser = dbUser;
        return this;
    }

    /**
     * Get dbUser
     *
     * @return dbUser
     */
    @javax.annotation.Nullable
    public String getDbUser() {
        return dbUser;
    }

    public void setDbUser(String dbUser) {
        this.dbUser = dbUser;
    }

    public RedshiftConfigOut workgroupName(String workgroupName) {
        this.workgroupName = workgroupName;
        return this;
    }

    /**
     * Get workgroupName
     *
     * @return workgroupName
     */
    @javax.annotation.Nullable
    public String getWorkgroupName() {
        return workgroupName;
    }

    public void setWorkgroupName(String workgroupName) {
        this.workgroupName = workgroupName;
    }

    public RedshiftConfigOut dbName(String dbName) {
        this.dbName = dbName;
        return this;
    }

    /**
     * Database name.
     *
     * <p>Only required if not using transformations.
     *
     * @return dbName
     */
    @javax.annotation.Nullable
    public String getDbName() {
        return dbName;
    }

    public void setDbName(String dbName) {
        this.dbName = dbName;
    }

    public RedshiftConfigOut schemaName(String schemaName) {
        this.schemaName = schemaName;
        return this;
    }

    /**
     * Schema name.
     *
     * <p>Only used if not using transformations.
     *
     * @return schemaName
     */
    @javax.annotation.Nullable
    public String getSchemaName() {
        return schemaName;
    }

    public void setSchemaName(String schemaName) {
        this.schemaName = schemaName;
    }

    public RedshiftConfigOut tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    /**
     * Table name.
     *
     * <p>Only required if not using transformations.
     *
     * @return tableName
     */
    @javax.annotation.Nullable
    public String getTableName() {
        return tableName;
    }

    public void setTableName(String tableName) {
        this.tableName = tableName;
    }

    /**
     * Create an instance of RedshiftConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RedshiftConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RedshiftConfigOut
     */
    public static RedshiftConfigOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RedshiftConfigOut.class);
    }

    /**
     * Convert an instance of RedshiftConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
