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
import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class SinkHttpConfigIn {
    @JsonProperty private URI url;
    @JsonProperty private Map<String, String> headers;
    @JsonProperty private String key;

    public SinkHttpConfigIn() {}

    public SinkHttpConfigIn url(URI url) {
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

    public SinkHttpConfigIn headers(Map<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public SinkHttpConfigIn putHeadersItem(String key, String headersItem) {
        if (this.headers == null) {
            this.headers = new HashMap<>();
        }
        this.headers.put(key, headersItem);

        return this;
    }

    /**
     * Get headers
     *
     * @return headers
     */
    @javax.annotation.Nullable
    public Map<String, String> getHeaders() {
        return headers;
    }

    public void setHeaders(Map<String, String> headers) {
        this.headers = headers;
    }

    public SinkHttpConfigIn key(String key) {
        this.key = key;
        return this;
    }

    /**
     * Get key
     *
     * @return key
     */
    @javax.annotation.Nullable
    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    /**
     * Create an instance of SinkHttpConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SinkHttpConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SinkHttpConfigIn
     */
    public static SinkHttpConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkHttpConfigIn.class);
    }

    /**
     * Convert an instance of SinkHttpConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
