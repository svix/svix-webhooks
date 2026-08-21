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
public class AzureBlobStorageConfigIn {
    @JsonProperty private String container;
    @JsonProperty private String account;
    @JsonProperty private String accessKey;

    public AzureBlobStorageConfigIn() {}

    public AzureBlobStorageConfigIn container(String container) {
        this.container = container;
        return this;
    }

    /**
     * Get container
     *
     * @return container
     */
    @javax.annotation.Nonnull
    public String getContainer() {
        return container;
    }

    public void setContainer(String container) {
        this.container = container;
    }

    public AzureBlobStorageConfigIn account(String account) {
        this.account = account;
        return this;
    }

    /**
     * Get account
     *
     * @return account
     */
    @javax.annotation.Nonnull
    public String getAccount() {
        return account;
    }

    public void setAccount(String account) {
        this.account = account;
    }

    public AzureBlobStorageConfigIn accessKey(String accessKey) {
        this.accessKey = accessKey;
        return this;
    }

    /**
     * Access key.
     *
     * <p>Currently a required field, but marked as optional because we may add different
     * authentication in the future.
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
     * Create an instance of AzureBlobStorageConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of AzureBlobStorageConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     AzureBlobStorageConfigIn
     */
    public static AzureBlobStorageConfigIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, AzureBlobStorageConfigIn.class);
    }

    /**
     * Convert an instance of AzureBlobStorageConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
