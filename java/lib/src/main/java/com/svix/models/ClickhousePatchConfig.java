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
public class ClickhousePatchConfig {
    @JsonProperty private String database;
    @JsonProperty private String password;
    @JsonProperty private String tableName;
    @JsonProperty private URI url;
    @JsonProperty private String username;

    public ClickhousePatchConfig() {}

    public ClickhousePatchConfig database(String database) {
        this.database = database;
        return this;
    }

    /**
     * Get database
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

    public ClickhousePatchConfig password(String password) {
        this.password = password;
        return this;
    }

    /**
     * Get password
     *
     * @return password
     */
    @javax.annotation.Nullable
    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }

    public ClickhousePatchConfig tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    /**
     * Get tableName
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

    public ClickhousePatchConfig url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nullable
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    public ClickhousePatchConfig username(String username) {
        this.username = username;
        return this;
    }

    /**
     * Get username
     *
     * @return username
     */
    @javax.annotation.Nullable
    public String getUsername() {
        return username;
    }

    public void setUsername(String username) {
        this.username = username;
    }

    /**
     * Create an instance of ClickhousePatchConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClickhousePatchConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ClickhousePatchConfig
     */
    public static ClickhousePatchConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClickhousePatchConfig.class);
    }

    /**
     * Convert an instance of ClickhousePatchConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
