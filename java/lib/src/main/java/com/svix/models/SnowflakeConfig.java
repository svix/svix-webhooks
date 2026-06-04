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
public class SnowflakeConfig {
    @JsonProperty private String accountIdentifier;
    @JsonProperty private String dbName;
    @JsonProperty private String privateKey;
    @JsonProperty private String schemaName;
    @JsonProperty private String tableName;
    @JsonProperty private String userId;

    public SnowflakeConfig() {}

    public SnowflakeConfig accountIdentifier(String accountIdentifier) {
        this.accountIdentifier = accountIdentifier;
        return this;
    }

    /**
     * Snowflake account identifier, which includes both the organization and account IDs separated
     * by a hyphen.
     *
     * @return accountIdentifier
     */
    @javax.annotation.Nonnull
    public String getAccountIdentifier() {
        return accountIdentifier;
    }

    public void setAccountIdentifier(String accountIdentifier) {
        this.accountIdentifier = accountIdentifier;
    }

    public SnowflakeConfig dbName(String dbName) {
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

    public SnowflakeConfig privateKey(String privateKey) {
        this.privateKey = privateKey;
        return this;
    }

    /**
     * PEM-encoded private key used for signing token-based requests to the Snowflake API.
     *
     * <p>Beginning/end delimiters are not required.
     *
     * @return privateKey
     */
    @javax.annotation.Nonnull
    public String getPrivateKey() {
        return privateKey;
    }

    public void setPrivateKey(String privateKey) {
        this.privateKey = privateKey;
    }

    public SnowflakeConfig schemaName(String schemaName) {
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

    public SnowflakeConfig tableName(String tableName) {
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

    public SnowflakeConfig userId(String userId) {
        this.userId = userId;
        return this;
    }

    /**
     * The Snowflake user id.
     *
     * @return userId
     */
    @javax.annotation.Nonnull
    public String getUserId() {
        return userId;
    }

    public void setUserId(String userId) {
        this.userId = userId;
    }

    /**
     * Create an instance of SnowflakeConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SnowflakeConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to SnowflakeConfig
     */
    public static SnowflakeConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SnowflakeConfig.class);
    }

    /**
     * Convert an instance of SnowflakeConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
