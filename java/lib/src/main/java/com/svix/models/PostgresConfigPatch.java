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
public class PostgresConfigPatch {
    @JsonProperty private String url;
    @JsonProperty private String password;
    @JsonProperty private String tableName;
    @JsonProperty private String sslRootCert;

    public PostgresConfigPatch() {}

    public PostgresConfigPatch url(String url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nullable
    public String getUrl() {
        return url;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public PostgresConfigPatch password(String password) {
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

    public PostgresConfigPatch tableName(String tableName) {
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

    public PostgresConfigPatch sslRootCert(String sslRootCert) {
        this.sslRootCert = sslRootCert;
        return this;
    }

    /**
     * Get sslRootCert
     *
     * @return sslRootCert
     */
    @javax.annotation.Nullable
    public String getSslRootCert() {
        return sslRootCert;
    }

    public void setSslRootCert(String sslRootCert) {
        this.sslRootCert = sslRootCert;
    }

    /**
     * Create an instance of PostgresConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PostgresConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PostgresConfigPatch
     */
    public static PostgresConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PostgresConfigPatch.class);
    }

    /**
     * Convert an instance of PostgresConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
