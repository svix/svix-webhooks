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
public class McpEventsUnsubscribeIn {
    @JsonProperty private String name;
    @JsonProperty private Object arguments;
    @JsonProperty private McpEventsUnsubscribeDeliveryIn delivery;

    public McpEventsUnsubscribeIn() {}

    public McpEventsUnsubscribeIn name(String name) {
        this.name = name;
        return this;
    }

    /**
     * The event the subscription was made for.
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

    public McpEventsUnsubscribeIn arguments(Object arguments) {
        this.arguments = arguments;
        return this;
    }

    /**
     * The filter arguments the subscription was made with, as given to `events/subscribe`. See
     * [Stop a
     * subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription) in
     * OpenAI's MCP Events docs.
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

    public McpEventsUnsubscribeIn delivery(McpEventsUnsubscribeDeliveryIn delivery) {
        this.delivery = delivery;
        return this;
    }

    /**
     * Get delivery
     *
     * @return delivery
     */
    @javax.annotation.Nonnull
    public McpEventsUnsubscribeDeliveryIn getDelivery() {
        return delivery;
    }

    public void setDelivery(McpEventsUnsubscribeDeliveryIn delivery) {
        this.delivery = delivery;
    }

    /**
     * Create an instance of McpEventsUnsubscribeIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of McpEventsUnsubscribeIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     McpEventsUnsubscribeIn
     */
    public static McpEventsUnsubscribeIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, McpEventsUnsubscribeIn.class);
    }

    /**
     * Convert an instance of McpEventsUnsubscribeIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
