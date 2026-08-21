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
public class ClickhouseConfigOut {
    @JsonProperty private URI url;
    @JsonProperty private String username;
    @JsonProperty private String database;
    @JsonProperty private String tableName;

    public ClickhouseConfigOut() {}

    public ClickhouseConfigOut url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
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

    public ClickhouseConfigOut username(String username) {
        this.username = username;
        return this;
    }

    /**
     * Get username
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

    public ClickhouseConfigOut database(String database) {
        this.database = database;
        return this;
    }

    /**
     * Get database
     *
     * @return database
     */
    @javax.annotation.Nonnull
    public String getDatabase() {
        return database;
    }

    public void setDatabase(String database) {
        this.database = database;
    }

    public ClickhouseConfigOut tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    /**
     * Get tableName
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

    /**
     * Create an instance of ClickhouseConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClickhouseConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ClickhouseConfigOut
     */
    public static ClickhouseConfigOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClickhouseConfigOut.class);
    }

    /**
     * Convert an instance of ClickhouseConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
