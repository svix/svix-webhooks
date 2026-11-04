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
public class PostgresConfigOut {
    @JsonProperty private String url;
    @JsonProperty private String tableName;
    @JsonProperty private String sslRootCert;

    public PostgresConfigOut() {}

    public PostgresConfigOut url(String url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public String getUrl() {
        return url;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public PostgresConfigOut tableName(String tableName) {
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

    public PostgresConfigOut sslRootCert(String sslRootCert) {
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
     * Create an instance of PostgresConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PostgresConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PostgresConfigOut
     */
    public static PostgresConfigOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PostgresConfigOut.class);
    }

    /**
     * Convert an instance of PostgresConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
