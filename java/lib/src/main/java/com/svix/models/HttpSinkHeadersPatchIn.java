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

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class HttpSinkHeadersPatchIn {
    @JsonProperty private Map<String, String> headers;

    public HttpSinkHeadersPatchIn() {}

    public HttpSinkHeadersPatchIn headers(Map<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public HttpSinkHeadersPatchIn putHeadersItem(String key, String headersItem) {
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
    @javax.annotation.Nonnull
    public Map<String, String> getHeaders() {
        return headers;
    }

    public void setHeaders(Map<String, String> headers) {
        this.headers = headers;
    }

    /**
     * Create an instance of HttpSinkHeadersPatchIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of HttpSinkHeadersPatchIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     HttpSinkHeadersPatchIn
     */
    public static HttpSinkHeadersPatchIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, HttpSinkHeadersPatchIn.class);
    }

    /**
     * Convert an instance of HttpSinkHeadersPatchIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
