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
import java.util.Map;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MessageIn {
    @JsonProperty private ApplicationIn application;
    @JsonProperty private Set<String> channels;
    @JsonProperty private OffsetDateTime deliverAt;
    @JsonProperty private String eventId;
    @JsonProperty private String eventType;
    @JsonProperty private String payload;
    @JsonProperty private Long payloadRetentionHours;
    @JsonProperty private Long payloadRetentionPeriod;
    @JsonProperty private Set<String> tags;
    @JsonProperty private Map<String, Object> transformationsParams;

    public MessageIn() {}

    public MessageIn application(ApplicationIn application) {
        this.application = application;
        return this;
    }

    /**
     * Optionally creates a new application alongside the message.
     *
     * <p>If the application id or uid that is used in the path already exists, this argument is
     * ignored.
     *
     * @return application
     */
    @javax.annotation.Nullable
    public ApplicationIn getApplication() {
        return application;
    }

    public void setApplication(ApplicationIn application) {
        this.application = application;
    }

    public MessageIn channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public MessageIn addChannelsItem(String channelsItem) {
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

    public MessageIn deliverAt(OffsetDateTime deliverAt) {
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

    public MessageIn eventId(String eventId) {
        this.eventId = eventId;
        return this;
    }

    /**
     * Optional unique identifier for the message
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

    public MessageIn eventType(String eventType) {
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

    public MessageIn payload(String payload) {
        this.payload = payload;
        return this;
    }

    /**
     * JSON payload to send as the request body of the webhook.
     *
     * <p>We also support sending non-JSON payloads. Please contact us for more information.
     *
     * @return payload
     */
    @javax.annotation.Nonnull
    public String getPayload() {
        return payload;
    }

    public void setPayload(String payload) {
        this.payload = payload;
    }

    public MessageIn payloadRetentionHours(Long payloadRetentionHours) {
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

    public MessageIn payloadRetentionPeriod(Long payloadRetentionPeriod) {
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

    public MessageIn tags(Set<String> tags) {
        this.tags = tags;
        return this;
    }

    public MessageIn addTagsItem(String tagsItem) {
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

    public MessageIn transformationsParams(Map<String, Object> transformationsParams) {
        this.transformationsParams = transformationsParams;
        return this;
    }

    /**
     * Extra parameters to pass to Transformations (for future use)
     *
     * @return transformationsParams
     */
    @javax.annotation.Nullable
    public Map<String, Object> getTransformationsParams() {
        return transformationsParams;
    }

    public void setTransformationsParams(Map<String, Object> transformationsParams) {
        this.transformationsParams = transformationsParams;
    }

    /**
     * Create an instance of MessageIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessageIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to MessageIn
     */
    public static MessageIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessageIn.class);
    }

    /**
     * Convert an instance of MessageIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
