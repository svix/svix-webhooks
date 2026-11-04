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
public class SegmentSinkConfigIn {
    @JsonProperty private URI host;
    @JsonProperty private String writeKey;

    public SegmentSinkConfigIn() {}

    public SegmentSinkConfigIn host(URI host) {
        this.host = host;
        return this;
    }

    /**
     * The Segment ingestion host.
     *
     * <p>This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com for
     * EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint under this
     * host.
     *
     * @return host
     */
    @javax.annotation.Nonnull
    public URI getHost() {
        return host;
    }

    public void setHost(URI host) {
        this.host = host;
    }

    public SegmentSinkConfigIn writeKey(String writeKey) {
        this.writeKey = writeKey;
        return this;
    }

    /**
     * The Segment source write key.
     *
     * <p>Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
     * `/v1/batch` request.
     *
     * @return writeKey
     */
    @javax.annotation.Nonnull
    public String getWriteKey() {
        return writeKey;
    }

    public void setWriteKey(String writeKey) {
        this.writeKey = writeKey;
    }

    /**
     * Create an instance of SegmentSinkConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SegmentSinkConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SegmentSinkConfigIn
     */
    public static SegmentSinkConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SegmentSinkConfigIn.class);
    }

    /**
     * Convert an instance of SegmentSinkConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
