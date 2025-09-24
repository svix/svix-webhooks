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
public class IngestMessageAttemptRecoveredEventData {
    @JsonProperty private String endpointId;
    @JsonProperty private MessageAttemptFailedData lastAttempt;
    @JsonProperty private String msgEventId;
    @JsonProperty private String msgId;
    @JsonProperty private String sourceId;

    public IngestMessageAttemptRecoveredEventData() {}

    public IngestMessageAttemptRecoveredEventData endpointId(String endpointId) {
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

    public IngestMessageAttemptRecoveredEventData lastAttempt(
            MessageAttemptFailedData lastAttempt) {
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

    public IngestMessageAttemptRecoveredEventData msgEventId(String msgEventId) {
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

    public IngestMessageAttemptRecoveredEventData msgId(String msgId) {
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

    public IngestMessageAttemptRecoveredEventData sourceId(String sourceId) {
        this.sourceId = sourceId;
        return this;
    }

    /**
     * The Source's ID.
     *
     * @return sourceId
     */
    @javax.annotation.Nonnull
    public String getSourceId() {
        return sourceId;
    }

    public void setSourceId(String sourceId) {
        this.sourceId = sourceId;
    }

    /**
     * Create an instance of IngestMessageAttemptRecoveredEventData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IngestMessageAttemptRecoveredEventData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     IngestMessageAttemptRecoveredEventData
     */
    public static IngestMessageAttemptRecoveredEventData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper()
                .readValue(jsonString, IngestMessageAttemptRecoveredEventData.class);
    }

    /**
     * Convert an instance of IngestMessageAttemptRecoveredEventData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
