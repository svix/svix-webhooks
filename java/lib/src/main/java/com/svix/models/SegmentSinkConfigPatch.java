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
public class SegmentSinkConfigPatch {
    @JsonProperty private URI host;
    @JsonProperty private String writeKey;

    public SegmentSinkConfigPatch() {}

    public SegmentSinkConfigPatch host(URI host) {
        this.host = host;
        return this;
    }

    /**
     * Get host
     *
     * @return host
     */
    @javax.annotation.Nullable
    public URI getHost() {
        return host;
    }

    public void setHost(URI host) {
        this.host = host;
    }

    public SegmentSinkConfigPatch writeKey(String writeKey) {
        this.writeKey = writeKey;
        return this;
    }

    /**
     * Get writeKey
     *
     * @return writeKey
     */
    @javax.annotation.Nullable
    public String getWriteKey() {
        return writeKey;
    }

    public void setWriteKey(String writeKey) {
        this.writeKey = writeKey;
    }

    /**
     * Create an instance of SegmentSinkConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SegmentSinkConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SegmentSinkConfigPatch
     */
    public static SegmentSinkConfigPatch fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SegmentSinkConfigPatch.class);
    }

    /**
     * Convert an instance of SegmentSinkConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
