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
public class EventBridgeConfigIn {
    @JsonProperty private String eventBusName;
    @JsonProperty private String detailType;
    @JsonProperty private String accessKeyId;
    @JsonProperty private String secretAccessKey;
    @JsonProperty private String roleArn;
    @JsonProperty private String externalId;
    @JsonProperty private String region;

    public EventBridgeConfigIn() {}

    public EventBridgeConfigIn eventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
        return this;
    }

    /**
     * The name or ARN of the event bus to receive the event
     *
     * @return eventBusName
     */
    @javax.annotation.Nonnull
    public String getEventBusName() {
        return eventBusName;
    }

    public void setEventBusName(String eventBusName) {
        this.eventBusName = eventBusName;
    }

    public EventBridgeConfigIn detailType(String detailType) {
        this.detailType = detailType;
        return this;
    }

    /**
     * Free-form string, with a maximum of 128 characters
     *
     * @return detailType
     */
    @javax.annotation.Nullable
    public String getDetailType() {
        return detailType;
    }

    public void setDetailType(String detailType) {
        this.detailType = detailType;
    }

    public EventBridgeConfigIn accessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
        return this;
    }

    /**
     * Access key ID.
     *
     * <p>Required (along with `secret_access_key`) if `role_arn` is blank.
     *
     * @return accessKeyId
     */
    @javax.annotation.Nullable
    public String getAccessKeyId() {
        return accessKeyId;
    }

    public void setAccessKeyId(String accessKeyId) {
        this.accessKeyId = accessKeyId;
    }

    public EventBridgeConfigIn secretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
        return this;
    }

    /**
     * Secret access key.
     *
     * <p>Required (along with `access_key_id`) if `role_arn` is blank.
     *
     * @return secretAccessKey
     */
    @javax.annotation.Nullable
    public String getSecretAccessKey() {
        return secretAccessKey;
    }

    public void setSecretAccessKey(String secretAccessKey) {
        this.secretAccessKey = secretAccessKey;
    }

    public EventBridgeConfigIn roleArn(String roleArn) {
        this.roleArn = roleArn;
        return this;
    }

    /**
     * Role ARN for delegated authentication
     *
     * @return roleArn
     */
    @javax.annotation.Nullable
    public String getRoleArn() {
        return roleArn;
    }

    public void setRoleArn(String roleArn) {
        this.roleArn = roleArn;
    }

    public EventBridgeConfigIn externalId(String externalId) {
        this.externalId = externalId;
        return this;
    }

    /**
     * Shared secret passed as the STS ExternalId.
     *
     * <p>Can only be set if `role_arn` is Some
     *
     * @return externalId
     */
    @javax.annotation.Nullable
    public String getExternalId() {
        return externalId;
    }

    public void setExternalId(String externalId) {
        this.externalId = externalId;
    }

    public EventBridgeConfigIn region(String region) {
        this.region = region;
        return this;
    }

    /**
     * The region of the EventBridge bus.
     *
     * <p>Currently a required field, but marked as optional because we may infer it from other
     * fields in the future.
     *
     * @return region
     */
    @javax.annotation.Nullable
    public String getRegion() {
        return region;
    }

    public void setRegion(String region) {
        this.region = region;
    }

    /**
     * Create an instance of EventBridgeConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventBridgeConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventBridgeConfigIn
     */
    public static EventBridgeConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventBridgeConfigIn.class);
    }

    /**
     * Convert an instance of EventBridgeConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
