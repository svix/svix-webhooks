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
public class MessageAttemptFailingEventData {
    @JsonProperty private String appId;
    @JsonProperty private String appUid;
    @JsonProperty private String endpointId;
    @JsonProperty private MessageAttemptFailedData lastAttempt;
    @JsonProperty private String msgEventId;
    @JsonProperty private String msgId;

    public MessageAttemptFailingEventData() {}

    public MessageAttemptFailingEventData appId(String appId) {
        this.appId = appId;
        return this;
    }

    /**
     * The Application's ID.
     *
     * @return appId
     */
    @javax.annotation.Nonnull
    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }

    public MessageAttemptFailingEventData appUid(String appUid) {
        this.appUid = appUid;
        return this;
    }

    /**
     * The Application's UID.
     *
     * @return appUid
     */
    @javax.annotation.Nullable
    public String getAppUid() {
        return appUid;
    }

    public void setAppUid(String appUid) {
        this.appUid = appUid;
    }

    public MessageAttemptFailingEventData endpointId(String endpointId) {
        this.endpointId = endpointId;
        return this;
    }

    /**
     * The Endpoint's ID.
     *
     * @return endpointId
     */
    @javax.annotation.Nonnull
    public String getEndpointId() {
        return endpointId;
    }

    public void setEndpointId(String endpointId) {
        this.endpointId = endpointId;
    }

    public MessageAttemptFailingEventData lastAttempt(MessageAttemptFailedData lastAttempt) {
        this.lastAttempt = lastAttempt;
        return this;
    }

    /**
     * Get lastAttempt
     *
     * @return lastAttempt
     */
    @javax.annotation.Nonnull
    public MessageAttemptFailedData getLastAttempt() {
        return lastAttempt;
    }

    public void setLastAttempt(MessageAttemptFailedData lastAttempt) {
        this.lastAttempt = lastAttempt;
    }

    public MessageAttemptFailingEventData msgEventId(String msgEventId) {
        this.msgEventId = msgEventId;
        return this;
    }

    /**
     * The Message's UID.
     *
     * @return msgEventId
     */
    @javax.annotation.Nullable
    public String getMsgEventId() {
        return msgEventId;
    }

    public void setMsgEventId(String msgEventId) {
        this.msgEventId = msgEventId;
    }

    public MessageAttemptFailingEventData msgId(String msgId) {
        this.msgId = msgId;
        return this;
    }

    /**
     * The Message's ID.
     *
     * @return msgId
     */
    @javax.annotation.Nonnull
    public String getMsgId() {
        return msgId;
    }

    public void setMsgId(String msgId) {
        this.msgId = msgId;
    }

    /**
     * Create an instance of MessageAttemptFailingEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptFailingEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptFailingEventData
     */
    public static MessageAttemptFailingEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptFailingEventData.class);
    }

    /**
     * Convert an instance of MessageAttemptFailingEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
