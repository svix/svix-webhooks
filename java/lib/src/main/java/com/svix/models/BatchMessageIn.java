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
import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class BatchMessageIn {
    @JsonProperty private String eventId;
    @JsonProperty private String eventType;
    @JsonProperty private Object payload;
    @JsonProperty private Set<String> channels;
    @JsonProperty private Set<String> tags;
    @JsonProperty private Object transformationsParams;
    @JsonProperty private OffsetDateTime deliverAt;
    @JsonProperty private Long payloadRetentionPeriod;
    @JsonProperty private Long payloadRetentionHours;

    public BatchMessageIn() {}

    public BatchMessageIn eventId(String eventId) {
        this.eventId = eventId;
        return this;
    }

    /**
     * Optional unique identifier for the message
     *
     * <p>This should not contain the '.' character
     *
     * @return eventId
     */
    @javax.annotation.Nullable
    public String getEventId() {
        return eventId;
    }

    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public BatchMessageIn eventType(String eventType) {
        this.eventType = eventType;
        return this;
    }

    /**
     * The event type's name
     *
     * @return eventType
     */
    @javax.annotation.Nonnull
    public String getEventType() {
        return eventType;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public BatchMessageIn payload(Object payload) {
        this.payload = payload;
        return this;
    }

    /**
     * JSON payload to send as the request body of the webhook.
     *
     * <p>We also support sending non-JSON payloads. Please contact us for more information.
     *
     * <p>The serialized byte-length of this value should be under 1MiB. Please contact us if you
     * need to send payloads large than 1MiB.
     *
     * @return payload
     */
    @javax.annotation.Nonnull
    public Object getPayload() {
        return payload;
    }

    public void setPayload(Object payload) {
        this.payload = payload;
    }

    public BatchMessageIn channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public BatchMessageIn addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new LinkedHashSet<>();
        }
        this.channels.add(channelsItem);

        return this;
    }

    /**
     * List of free-form identifiers that endpoints can filter by
     *
     * @return channels
     */
    @javax.annotation.Nullable
    public Set<String> getChannels() {
        return channels;
    }

    public void setChannels(Set<String> channels) {
        this.channels = channels;
    }

    public BatchMessageIn tags(Set<String> tags) {
        this.tags = tags;
        return this;
    }

    public BatchMessageIn addTagsItem(String tagsItem) {
        if (this.tags == null) {
            this.tags = new LinkedHashSet<>();
        }
        this.tags.add(tagsItem);

        return this;
    }

    /**
     * List of free-form tags that can be filtered by when listing messages
     *
     * @return tags
     */
    @javax.annotation.Nullable
    public Set<String> getTags() {
        return tags;
    }

    public void setTags(Set<String> tags) {
        this.tags = tags;
    }

    public BatchMessageIn transformationsParams(Object transformationsParams) {
        this.transformationsParams = transformationsParams;
        return this;
    }

    /**
     * Extra parameters to pass to Transformations (for future use)
     *
     * @return transformationsParams
     */
    @javax.annotation.Nullable
    public Object getTransformationsParams() {
        return transformationsParams;
    }

    public void setTransformationsParams(Object transformationsParams) {
        this.transformationsParams = transformationsParams;
    }

    public BatchMessageIn deliverAt(OffsetDateTime deliverAt) {
        this.deliverAt = deliverAt;
        return this;
    }

    /**
     * The date and time at which the message will be delivered.
     *
     * <p>Note that this time is best-effort-only. Must be at least one minute and no more than 24
     * hours in the future.
     *
     * @return deliverAt
     */
    @javax.annotation.Nullable
    public OffsetDateTime getDeliverAt() {
        return deliverAt;
    }

    public void setDeliverAt(OffsetDateTime deliverAt) {
        this.deliverAt = deliverAt;
    }

    public BatchMessageIn payloadRetentionPeriod(Long payloadRetentionPeriod) {
        this.payloadRetentionPeriod = payloadRetentionPeriod;
        return this;
    }

    /**
     * Optional number of days to retain the message payload. Defaults to 90. Note that this is
     * mutually exclusive with `payloadRetentionHours`.
     *
     * @return payloadRetentionPeriod
     */
    @javax.annotation.Nullable
    public Long getPayloadRetentionPeriod() {
        return payloadRetentionPeriod;
    }

    public void setPayloadRetentionPeriod(Long payloadRetentionPeriod) {
        this.payloadRetentionPeriod = payloadRetentionPeriod;
    }

    public BatchMessageIn payloadRetentionHours(Long payloadRetentionHours) {
        this.payloadRetentionHours = payloadRetentionHours;
        return this;
    }

    /**
     * Optional number of hours to retain the message payload. Note that this is mutually exclusive
     * with `payloadRetentionPeriod`.
     *
     * @return payloadRetentionHours
     */
    @javax.annotation.Nullable
    public Long getPayloadRetentionHours() {
        return payloadRetentionHours;
    }

    public void setPayloadRetentionHours(Long payloadRetentionHours) {
        this.payloadRetentionHours = payloadRetentionHours;
    }

    /**
     * Create an instance of BatchMessageIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BatchMessageIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to BatchMessageIn
     */
    public static BatchMessageIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BatchMessageIn.class);
    }

    /**
     * Convert an instance of BatchMessageIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
