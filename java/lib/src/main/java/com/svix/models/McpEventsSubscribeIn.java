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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class McpEventsSubscribeIn {
    @JsonProperty private String name;
    @JsonProperty private Object arguments;
    @JsonProperty private McpEventsSubscribeDeliveryIn delivery;
    @JsonProperty private Long ttlMs;
    @JsonProperty private Integer secretGracePeriodSeconds;
    @JsonProperty private EndpointChallengeMode challengeMode;
    @JsonProperty private Set<String> channels;

    public McpEventsSubscribeIn() {}

    public McpEventsSubscribeIn name(String name) {
        this.name = name;
        return this;
    }

    /**
     * The event to subscribe to. It must be an existing event type.
     *
     * @return name
     */
    @javax.annotation.Nonnull
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public McpEventsSubscribeIn arguments(Object arguments) {
        this.arguments = arguments;
        return this;
    }

    /**
     * The filter arguments of the `events/subscribe` request, as described by the event's
     * `inputSchema` (e.g. `{"document_id": "doc_123"}`). Together with `name` and the callback URL
     * they identify the subscription
     *
     * <p>They're stored in the endpoint's `mcpEventArguments` metadata, but aren't used to filter
     * messages; use `channels` for that.
     *
     * <p>See [Create a
     * subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
     * in OpenAI's MCP Events docs.
     *
     * @return arguments
     */
    @javax.annotation.Nullable
    public Object getArguments() {
        return arguments;
    }

    public void setArguments(Object arguments) {
        this.arguments = arguments;
    }

    public McpEventsSubscribeIn delivery(McpEventsSubscribeDeliveryIn delivery) {
        this.delivery = delivery;
        return this;
    }

    /**
     * Get delivery
     *
     * @return delivery
     */
    @javax.annotation.Nonnull
    public McpEventsSubscribeDeliveryIn getDelivery() {
        return delivery;
    }

    public void setDelivery(McpEventsSubscribeDeliveryIn delivery) {
        this.delivery = delivery;
    }

    public McpEventsSubscribeIn ttlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
        return this;
    }

    /**
     * The requested lifetime of the subscription, in milliseconds. Omit for the default of 7 days,
     * or set to `null` for a subscription that never expires.
     *
     * @return ttlMs
     */
    @javax.annotation.Nullable
    public Long getTtlMs() {
        return ttlMs;
    }

    public void setTtlMs(Long ttlMs) {
        this.ttlMs = ttlMs;
    }

    public McpEventsSubscribeIn secretGracePeriodSeconds(Integer secretGracePeriodSeconds) {
        this.secretGracePeriodSeconds = secretGracePeriodSeconds;
        return this;
    }

    /**
     * How long the previous secret keeps signing when a refresh changes it, in seconds.
     *
     * <p>Valid values are between 0 (immediate expiry) and 7 days. The default is 5 minutes.
     *
     * @return secretGracePeriodSeconds
     */
    @javax.annotation.Nullable
    public Integer getSecretGracePeriodSeconds() {
        return secretGracePeriodSeconds;
    }

    public void setSecretGracePeriodSeconds(Integer secretGracePeriodSeconds) {
        this.secretGracePeriodSeconds = secretGracePeriodSeconds;
    }

    public McpEventsSubscribeIn challengeMode(EndpointChallengeMode challengeMode) {
        this.challengeMode = challengeMode;
        return this;
    }

    /**
     * How the callback must respond to the verification challenge.
     *
     * @return challengeMode
     */
    @javax.annotation.Nullable
    public EndpointChallengeMode getChallengeMode() {
        return challengeMode;
    }

    public void setChallengeMode(EndpointChallengeMode challengeMode) {
        this.challengeMode = challengeMode;
    }

    public McpEventsSubscribeIn channels(Set<String> channels) {
        this.channels = channels;
        return this;
    }

    public McpEventsSubscribeIn addChannelsItem(String channelsItem) {
        if (this.channels == null) {
            this.channels = new LinkedHashSet<>();
        }
        this.channels.add(channelsItem);

        return this;
    }

    /**
     * Only send messages with one of these channels to the subscription, for example ones derived
     * from `arguments`. Omit to send all messages of the event type.
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

    /**
     * Create an instance of McpEventsSubscribeIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of McpEventsSubscribeIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     McpEventsSubscribeIn
     */
    public static McpEventsSubscribeIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, McpEventsSubscribeIn.class);
    }

    /**
     * Convert an instance of McpEventsSubscribeIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
