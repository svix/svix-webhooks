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
public class SnowflakeConfigPatch {
    @JsonProperty private String privateKey;
    @JsonProperty private String accountIdentifier;
    @JsonProperty private String userId;
    @JsonProperty private String dbName;
    @JsonProperty private String schemaName;
    @JsonProperty private String tableName;

    public SnowflakeConfigPatch() {}

    public SnowflakeConfigPatch privateKey(String privateKey) {
        this.privateKey = privateKey;
        return this;
    }

    /**
     * Get privateKey
     *
     * @return privateKey
     */
    @javax.annotation.Nullable
    public String getPrivateKey() {
        return privateKey;
    }

    public void setPrivateKey(String privateKey) {
        this.privateKey = privateKey;
    }

    public SnowflakeConfigPatch accountIdentifier(String accountIdentifier) {
        this.accountIdentifier = accountIdentifier;
        return this;
    }

    /**
     * Get accountIdentifier
     *
     * @return accountIdentifier
     */
    @javax.annotation.Nullable
    public String getAccountIdentifier() {
        return accountIdentifier;
    }

    public void setAccountIdentifier(String accountIdentifier) {
        this.accountIdentifier = accountIdentifier;
    }

    public SnowflakeConfigPatch userId(String userId) {
        this.userId = userId;
        return this;
    }

    /**
     * Get userId
     *
     * @return userId
     */
    @javax.annotation.Nullable
    public String getUserId() {
        return userId;
    }

    public void setUserId(String userId) {
        this.userId = userId;
    }

    public SnowflakeConfigPatch dbName(String dbName) {
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

    public SnowflakeConfigPatch schemaName(String schemaName) {
        this.schemaName = schemaName;
        return this;
    }

    /**
     * Schema name.
     *
     * <p>Only required if not using transformations.
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

    public SnowflakeConfigPatch tableName(String tableName) {
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
     * Create an instance of SnowflakeConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SnowflakeConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SnowflakeConfigPatch
     */
    public static SnowflakeConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SnowflakeConfigPatch.class);
    }

    /**
     * Convert an instance of SnowflakeConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
