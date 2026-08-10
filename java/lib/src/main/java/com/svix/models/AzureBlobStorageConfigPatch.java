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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class AzureBlobStorageConfigPatch {
    @JsonProperty private String container;
    @JsonProperty private String account;
    @JsonProperty private String accessKey;

    public AzureBlobStorageConfigPatch() {}

    public AzureBlobStorageConfigPatch container(String container) {
        this.container = container;
        return this;
    }

    /**
     * Get container
     *
     * @return container
     */
    @javax.annotation.Nullable
    public String getContainer() {
        return container;
    }

    public void setContainer(String container) {
        this.container = container;
    }

    public AzureBlobStorageConfigPatch account(String account) {
        this.account = account;
        return this;
    }

    /**
     * Get account
     *
     * @return account
     */
    @javax.annotation.Nullable
    public String getAccount() {
        return account;
    }

    public void setAccount(String account) {
        this.account = account;
    }

    public AzureBlobStorageConfigPatch accessKey(String accessKey) {
        this.accessKey = accessKey;
        return this;
    }

    /**
     * Get accessKey
     *
     * @return accessKey
     */
    @javax.annotation.Nullable
    public String getAccessKey() {
        return accessKey;
    }

    public void setAccessKey(String accessKey) {
        this.accessKey = accessKey;
    }

    /**
     * Create an instance of AzureBlobStorageConfigPatch given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AzureBlobStorageConfigPatch
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AzureBlobStorageConfigPatch
     */
    public static AzureBlobStorageConfigPatch fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AzureBlobStorageConfigPatch.class);
    }

    /**
     * Convert an instance of AzureBlobStorageConfigPatch to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
