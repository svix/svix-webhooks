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

import java.time.OffsetDateTime;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MessageAttemptLog {
    @JsonProperty private String appId;
    @JsonProperty private String appUid;
    @JsonProperty private Long attemptCount;
    @JsonProperty private OffsetDateTime attemptEnd;
    @JsonProperty private String attemptId;
    @JsonProperty private OffsetDateTime attemptStart;
    @JsonProperty private String endpointId;
    @JsonProperty private String eventType;
    @JsonProperty private HttpAttemptTimes httpTimes;
    @JsonProperty private OffsetDateTime msgCreated;
    @JsonProperty private String msgEventId;
    @JsonProperty private String msgId;
    @JsonProperty private String orgId;
    @JsonProperty private Short responseStatusCode;
    @JsonProperty private MessageStatus status;

    public MessageAttemptLog() {}

    public MessageAttemptLog appId(String appId) {
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

    public MessageAttemptLog appUid(String appUid) {
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

    public MessageAttemptLog attemptCount(Long attemptCount) {
        this.attemptCount = attemptCount;
        return this;
    }

    /**
     * Get attemptCount
     *
     * @return attemptCount
     */
    @javax.annotation.Nonnull
    public Long getAttemptCount() {
        return attemptCount;
    }

    public void setAttemptCount(Long attemptCount) {
        this.attemptCount = attemptCount;
    }

    public MessageAttemptLog attemptEnd(OffsetDateTime attemptEnd) {
        this.attemptEnd = attemptEnd;
        return this;
    }

    /**
     * Get attemptEnd
     *
     * @return attemptEnd
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getAttemptEnd() {
        return attemptEnd;
    }

    public void setAttemptEnd(OffsetDateTime attemptEnd) {
        this.attemptEnd = attemptEnd;
    }

    public MessageAttemptLog attemptId(String attemptId) {
        this.attemptId = attemptId;
        return this;
    }

    /**
     * The MessageAttempt's ID.
     *
     * @return attemptId
     */
    @javax.annotation.Nonnull
    public String getAttemptId() {
        return attemptId;
    }

    public void setAttemptId(String attemptId) {
        this.attemptId = attemptId;
    }

    public MessageAttemptLog attemptStart(OffsetDateTime attemptStart) {
        this.attemptStart = attemptStart;
        return this;
    }

    /**
     * Get attemptStart
     *
     * @return attemptStart
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getAttemptStart() {
        return attemptStart;
    }

    public void setAttemptStart(OffsetDateTime attemptStart) {
        this.attemptStart = attemptStart;
    }

    public MessageAttemptLog endpointId(String endpointId) {
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

    public MessageAttemptLog eventType(String eventType) {
        this.eventType = eventType;
        return this;
    }

    /**
     * The event type's name
     *
     * @return eventType
     */
    @javax.annotation.Nullable
    public String getEventType() {
        return eventType;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public MessageAttemptLog httpTimes(HttpAttemptTimes httpTimes) {
        this.httpTimes = httpTimes;
        return this;
    }

    /**
     * Get httpTimes
     *
     * @return httpTimes
     */
    @javax.annotation.Nullable
    public HttpAttemptTimes getHttpTimes() {
        return httpTimes;
    }

    public void setHttpTimes(HttpAttemptTimes httpTimes) {
        this.httpTimes = httpTimes;
    }

    public MessageAttemptLog msgCreated(OffsetDateTime msgCreated) {
        this.msgCreated = msgCreated;
        return this;
    }

    /**
     * Get msgCreated
     *
     * @return msgCreated
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getMsgCreated() {
        return msgCreated;
    }

    public void setMsgCreated(OffsetDateTime msgCreated) {
        this.msgCreated = msgCreated;
    }

    public MessageAttemptLog msgEventId(String msgEventId) {
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

    public MessageAttemptLog msgId(String msgId) {
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

    public MessageAttemptLog orgId(String orgId) {
        this.orgId = orgId;
        return this;
    }

    /**
     * The Environment's ID.
     *
     * @return orgId
     */
    @javax.annotation.Nonnull
    public String getOrgId() {
        return orgId;
    }

    public void setOrgId(String orgId) {
        this.orgId = orgId;
    }

    public MessageAttemptLog responseStatusCode(Short responseStatusCode) {
        this.responseStatusCode = responseStatusCode;
        return this;
    }

    /**
     * Get responseStatusCode
     *
     * @return responseStatusCode
     */
    @javax.annotation.Nonnull
    public Short getResponseStatusCode() {
        return responseStatusCode;
    }

    public void setResponseStatusCode(Short responseStatusCode) {
        this.responseStatusCode = responseStatusCode;
    }

    public MessageAttemptLog status(MessageStatus status) {
        this.status = status;
        return this;
    }

    /**
     * Get status
     *
     * @return status
     */
    @javax.annotation.Nonnull
    public MessageStatus getStatus() {
        return status;
    }

    public void setStatus(MessageStatus status) {
        this.status = status;
    }

    /**
     * Create an instance of MessageAttemptLog given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptLog
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptLog
     */
    public static MessageAttemptLog fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptLog.class);
    }

    /**
     * Convert an instance of MessageAttemptLog to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
