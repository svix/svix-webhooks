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
import java.time.OffsetDateTime;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MessageAttemptOut {
    @JsonProperty private String endpointId;
    @JsonProperty private String id;
    @JsonProperty private MessageOut msg;
    @JsonProperty private String msgId;
    @JsonProperty private String response;
    @JsonProperty private Long responseDurationMs;
    @JsonProperty private Short responseStatusCode;
    @JsonProperty private MessageStatus status;
    @JsonProperty private MessageStatusText statusText;
    @JsonProperty private OffsetDateTime timestamp;
    @JsonProperty private MessageAttemptTriggerType triggerType;
    @JsonProperty private URI url;

    public MessageAttemptOut() {}

    public MessageAttemptOut endpointId(String endpointId) {
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

    public MessageAttemptOut id(String id) {
        this.id = id;
        return this;
    }

    /**
     * The MessageAttempt's ID.
     *
     * @return id
     */
    @javax.annotation.Nonnull
    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public MessageAttemptOut msg(MessageOut msg) {
        this.msg = msg;
        return this;
    }

    /**
     * Get msg
     *
     * @return msg
     */
    @javax.annotation.Nullable
    public MessageOut getMsg() {
        return msg;
    }

    public void setMsg(MessageOut msg) {
        this.msg = msg;
    }

    public MessageAttemptOut msgId(String msgId) {
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

    public MessageAttemptOut response(String response) {
        this.response = response;
        return this;
    }

    /**
     * Get response
     *
     * @return response
     */
    @javax.annotation.Nonnull
    public String getResponse() {
        return response;
    }

    public void setResponse(String response) {
        this.response = response;
    }

    public MessageAttemptOut responseDurationMs(Long responseDurationMs) {
        this.responseDurationMs = responseDurationMs;
        return this;
    }

    /**
     * Response duration in milliseconds.
     *
     * @return responseDurationMs
     */
    @javax.annotation.Nonnull
    public Long getResponseDurationMs() {
        return responseDurationMs;
    }

    public void setResponseDurationMs(Long responseDurationMs) {
        this.responseDurationMs = responseDurationMs;
    }

    public MessageAttemptOut responseStatusCode(Short responseStatusCode) {
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

    public MessageAttemptOut status(MessageStatus status) {
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

    public MessageAttemptOut statusText(MessageStatusText statusText) {
        this.statusText = statusText;
        return this;
    }

    /**
     * Get statusText
     *
     * @return statusText
     */
    @javax.annotation.Nonnull
    public MessageStatusText getStatusText() {
        return statusText;
    }

    public void setStatusText(MessageStatusText statusText) {
        this.statusText = statusText;
    }

    public MessageAttemptOut timestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
        return this;
    }

    /**
     * Get timestamp
     *
     * @return timestamp
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(OffsetDateTime timestamp) {
        this.timestamp = timestamp;
    }

    public MessageAttemptOut triggerType(MessageAttemptTriggerType triggerType) {
        this.triggerType = triggerType;
        return this;
    }

    /**
     * Get triggerType
     *
     * @return triggerType
     */
    @javax.annotation.Nonnull
    public MessageAttemptTriggerType getTriggerType() {
        return triggerType;
    }

    public void setTriggerType(MessageAttemptTriggerType triggerType) {
        this.triggerType = triggerType;
    }

    public MessageAttemptOut url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * Get url
     *
     * @return url
     */
    @javax.annotation.Nonnull
    public URI getUrl() {
        return url;
    }

    public void setUrl(URI url) {
        this.url = url;
    }

    /**
     * Create an instance of MessageAttemptOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageAttemptOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessageAttemptOut
     */
    public static MessageAttemptOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageAttemptOut.class);
    }

    /**
     * Convert an instance of MessageAttemptOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
