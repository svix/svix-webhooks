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
public class PostgresConfigIn {
    @JsonProperty private String url;
    @JsonProperty private String password;
    @JsonProperty private String tableName;
    @JsonProperty private String sslRootCert;

    public PostgresConfigIn() {}

    public PostgresConfigIn url(String url) {
        this.url = url;
        return this;
    }

    /**
     * PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.
     *
     * <p>Do NOT embed a password here; use the `password` field instead.
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

    public PostgresConfigIn password(String password) {
        this.password = password;
        return this;
    }

    /**
     * Password for the connection.
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

    public PostgresConfigIn tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    /**
     * Table to insert into. May be schema-qualified (e.g. `public.events`).
     *
     * <p>Quote characters are not supported. Each dot-separated segment is automatically
     * double-quoted when the query is built, so `public.events` becomes `"public"."events"`.
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

    public PostgresConfigIn sslRootCert(String sslRootCert) {
        this.sslRootCert = sslRootCert;
        return this;
    }

    /**
     * PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.
     *
     * <p>Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca`
     * or `sslmode=verify-full`. Without it, only the built-in public roots are trusted.
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
     * Create an instance of PostgresConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of PostgresConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     PostgresConfigIn
     */
    public static PostgresConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, PostgresConfigIn.class);
    }

    /**
     * Convert an instance of PostgresConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
