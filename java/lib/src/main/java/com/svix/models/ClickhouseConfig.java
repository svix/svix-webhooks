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

import java.net.URI;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ClickhouseConfig {
    @JsonProperty private String database;
    @JsonProperty private String password;
    @JsonProperty private String tableName;
    @JsonProperty private URI url;
    @JsonProperty private String username;

    public ClickhouseConfig() {}

    public ClickhouseConfig database(String database) {
        this.database = database;
        return this;
    }

    /**
     * The Clickhouse database to connect to
     *
     * @return database
     */
    @javax.annotation.Nullable
    public String getDatabase() {
        return database;
    }

    public void setDatabase(String database) {
        this.database = database;
    }

    public ClickhouseConfig password(String password) {
        this.password = password;
        return this;
    }

    /**
     * Password to access Clickhouse
     *
     * @return password
     */
    @javax.annotation.Nonnull
    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }

    public ClickhouseConfig tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    /**
     * The Clickhouse table to write to
     *
     * @return tableName
     */
    @javax.annotation.Nonnull
    public String getTableName() {
        return tableName;
    }

    public void setTableName(String tableName) {
        this.tableName = tableName;
    }

    public ClickhouseConfig url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public ClickhouseConfig username(String username) {
        this.username = username;
        return this;
    }

    /**
     * Username to access Clickhouse
     *
     * @return username
     */
    @javax.annotation.Nonnull
    public String getUsername() {
        return username;
    }

    public void setUsername(String username) {
        this.username = username;
    }

    /**
     * Create an instance of ClickhouseConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClickhouseConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ClickhouseConfig
     */
    public static ClickhouseConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClickhouseConfig.class);
    }

    /**
     * Convert an instance of ClickhouseConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
