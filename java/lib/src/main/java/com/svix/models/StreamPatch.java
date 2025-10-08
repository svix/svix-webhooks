// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.MaybeUnset;
import com.svix.Utils;

import lombok.EqualsAndHashCode;
import lombok.ToString;

import java.util.HashMap;
import java.util.Map;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class StreamPatch {
    @JsonProperty private String description;
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private MaybeUnset<String> uid;

    public StreamPatch() {}

    public StreamPatch description(String description) {
        this.description = description;
        return this;
    }

    /**
     * The Stream's description.
     *
     * @return description
     */
    @javax.annotation.Nullable
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public StreamPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public StreamPatch putMetadataItem(String key, String metadataItem) {
        if (this.metadata == null) {
            this.metadata = new HashMap<>();
        }
        this.metadata.put(key, metadataItem);

        return this;
    }

    /**
     * Get metadata
     *
     * @return metadata
     */
    @javax.annotation.Nullable
    public Map<String, String> getMetadata() {
        return metadata;
    }

    public void setMetadata(Map<String, String> metadata) {
        this.metadata = metadata;
    }

    public StreamPatch uid(String uid) {
        this.uid = new MaybeUnset<>(uid);
        return this;
    }

    /**
     * An optional unique identifier for the stream.
     *
     * @return uid
     */
    @javax.annotation.Nullable
    public String getUid() {
        if (uid == null) {
            return null;
        }
        return uid.getValue();
    }

    public void setUid(String uid) {
        this.uid = new MaybeUnset<>(uid);
    }

    /**
     * Create an instance of StreamPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of StreamPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to StreamPatch
     */
    public static StreamPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, StreamPatch.class);
    }

    /**
     * Convert an instance of StreamPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
