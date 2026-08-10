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
public class SinkHttpConfigPatch {
    @JsonProperty private URI url;

    public SinkHttpConfigPatch() {}

    public SinkHttpConfigPatch url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nullable
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    /**
     * Create an instance of SinkHttpConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SinkHttpConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     SinkHttpConfigPatch
     */
    public static SinkHttpConfigPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkHttpConfigPatch.class);
    }

    /**
     * Convert an instance of SinkHttpConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
