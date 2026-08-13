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
public class RedshiftConfigIn {
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private String region;
    @JsonProperty private String clusterIdentifier;
    @JsonProperty private String dbUser;
    @JsonProperty private String workgroupName;
    @JsonProperty private String dbName;
    @JsonProperty private String schemaName;
    @JsonProperty private String tableName;

    public RedshiftConfigIn() {}

    public RedshiftConfigIn accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Access key ID.
     *
     * <p>Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     *
     * @return accessKeyId
     */
    @javax.annotation.Nullable
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public RedshiftConfigIn secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Secret access key.
     *
     * <p>Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     *
     * @return secretAccessKey
     */
    @javax.annotation.Nullable
    public String getSecretAccessKey() {
        return secretAccessKey;
    }

    public void setSecretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
    }

    public RedshiftConfigIn region(String region) {
        this.region = region;
        return this;
    }

    /**
     * The region of the Redshift DB.
     *
     * <p>Currently a required field, but marked as optional because we may infer it from other
     * fields in the future.
     *
     * @return region
     */
    @javax.annotation.Nullable
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    public RedshiftConfigIn clusterIdentifier(String clusterIdentifier) {
        this.clusterIdentifier = clusterIdentifier;
        return this;
    }

    /**
     * Required for provisioned clusters.
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

    public RedshiftConfigIn dbUser(String dbUser) {
        this.dbUser = dbUser;
        return this;
    }

    /**
     * Required for provisioned clusters.
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

    public RedshiftConfigIn workgroupName(String workgroupName) {
        this.workgroupName = workgroupName;
        return this;
    }

    /**
     * Required for Redshift Serverless.
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

    public RedshiftConfigIn dbName(String dbName) {
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

    public RedshiftConfigIn schemaName(String schemaName) {
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

    public RedshiftConfigIn tableName(String tableName) {
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
     * Create an instance of RedshiftConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RedshiftConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RedshiftConfigIn
     */
    public static RedshiftConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RedshiftConfigIn.class);
    }

    /**
     * Convert an instance of RedshiftConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
