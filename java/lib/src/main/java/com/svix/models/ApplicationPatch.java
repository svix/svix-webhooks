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
public class ApplicationPatch {
    @JsonProperty private Map<String, String> metadata;
    @JsonProperty private String name;
    @JsonProperty private MaybeUnset<Long> rateLimit;
    @JsonProperty private MaybeUnset<String> uid;

    public ApplicationPatch() {}

    public ApplicationPatch metadata(Map<String, String> metadata) {
        this.metadata = metadata;
        return this;
    }

    public ApplicationPatch putMetadataItem(String key, String metadataItem) {
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

    public ApplicationPatch name(String name) {
        this.name = name;
        return this;
    }

    /**
     * Get name
     *
     * @return name
     */
    @javax.annotation.Nullable
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public ApplicationPatch rateLimit(Long rateLimit) {
        this.rateLimit = new MaybeUnset<>(rateLimit);
        return this;
    }

    /**
     * Get rateLimit
     *
     * @return rateLimit
     */
    @javax.annotation.Nullable
    public Long getRateLimit() {
        if (rateLimit == null) {
            return null;
        }
        return rateLimit.getValue();
    }

    public void setRateLimit(Long rateLimit) {
        this.rateLimit = new MaybeUnset<>(rateLimit);
    }

    public ApplicationPatch uid(String uid) {
        this.uid = new MaybeUnset<>(uid);
        return this;
    }

    /**
     * The Application's UID.
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
     * Create an instance of ApplicationPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ApplicationPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ApplicationPatch
     */
    public static ApplicationPatch fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ApplicationPatch.class);
    }

    /**
     * Convert an instance of ApplicationPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
