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
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class EndpointHeadersOut {
    @JsonProperty private Map<String, String> headers;
    @JsonProperty private Set<String> sensitive;

    public EndpointHeadersOut() {}

    public EndpointHeadersOut headers(Map<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public EndpointHeadersOut putHeadersItem(String key, String headersItem) {
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

    public EndpointHeadersOut sensitive(Set<String> sensitive) {
        this.sensitive = sensitive;
        return this;
    }

    public EndpointHeadersOut addSensitiveItem(String sensitiveItem) {
        if (this.sensitive == null) {
            this.sensitive = new LinkedHashSet<>();
        }
        this.sensitive.add(sensitiveItem);

        return this;
    }

    /**
     * Get sensitive
     *
     * @return sensitive
     */
    @javax.annotation.Nonnull
    public Set<String> getSensitive() {
        return sensitive;
    }

    public void setSensitive(Set<String> sensitive) {
        this.sensitive = sensitive;
    }

    /**
     * Create an instance of EndpointHeadersOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EndpointHeadersOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EndpointHeadersOut
     */
    public static EndpointHeadersOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EndpointHeadersOut.class);
    }

    /**
     * Convert an instance of EndpointHeadersOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
