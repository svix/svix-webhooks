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
public class SinkOtelTracingConfigOut {
    @JsonProperty private URI url;
    @JsonProperty private EndpointHeadersOut headers;

    public SinkOtelTracingConfigOut() {}

    public SinkOtelTracingConfigOut url(URI url) {
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

    public SinkOtelTracingConfigOut headers(EndpointHeadersOut headers) {
        this.headers = headers;
        return this;
    }

    /**
     * Get headers
     *
     * @return headers
     */
    @javax.annotation.Nonnull
    public EndpointHeadersOut getHeaders() {
        return headers;
    }

    public void setHeaders(EndpointHeadersOut headers) {
        this.headers = headers;
    }

    /**
     * Create an instance of SinkOtelTracingConfigOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SinkOtelTracingConfigOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SinkOtelTracingConfigOut
     */
    public static SinkOtelTracingConfigOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkOtelTracingConfigOut.class);
    }

    /**
     * Convert an instance of SinkOtelTracingConfigOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
