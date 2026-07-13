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
    @JsonProperty private String name;
    @JsonProperty private MaybeUnset<Long> throttleRate;
    @JsonProperty private MaybeUnset<String> uid;
    @JsonProperty private Map<String, String> metadata;

    public ApplicationPatch() {}

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

    public ApplicationPatch throttleRate(Long throttleRate) {
        this.throttleRate = new MaybeUnset<>(throttleRate);
        return this;
    }

    /**
     * Maximum messages per second to send to this application.
     *
     * <p>Outgoing messages will be throttled to this rate.
     *
     * @return throttleRate
     */
    @javax.annotation.Nullable
    public Long getThrottleRate() {
        if (throttleRate == null) {
            return null;
        }
        return throttleRate.getValue();
    }

    public void setThrottleRate(Long throttleRate) {
        this.throttleRate = new MaybeUnset<>(throttleRate);
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
