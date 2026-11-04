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

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class McpEventsUnsubscribeDeliveryIn {
    @JsonProperty private McpEventsDeliveryMode mode;
    @JsonProperty private URI url;

    public McpEventsUnsubscribeDeliveryIn() {}

    public McpEventsUnsubscribeDeliveryIn mode(McpEventsDeliveryMode mode) {
        this.mode = mode;
        return this;
    }

    /**
     * How events are delivered. Only `webhook` is supported.
     *
     * @return mode
     */
    @javax.annotation.Nonnull
    public McpEventsDeliveryMode getMode() {
        return mode;
    }

    public void setMode(McpEventsDeliveryMode mode) {
        this.mode = mode;
    }

    public McpEventsUnsubscribeDeliveryIn url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * The callback URL the subscription was made with.
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
     * Create an instance of McpEventsUnsubscribeDeliveryIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of McpEventsUnsubscribeDeliveryIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     McpEventsUnsubscribeDeliveryIn
     */
    public static McpEventsUnsubscribeDeliveryIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, McpEventsUnsubscribeDeliveryIn.class);
    }

    /**
     * Convert an instance of McpEventsUnsubscribeDeliveryIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
