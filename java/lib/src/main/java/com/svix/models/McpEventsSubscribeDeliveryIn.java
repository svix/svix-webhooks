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
public class McpEventsSubscribeDeliveryIn {
    @JsonProperty private McpEventsDeliveryMode mode;
    @JsonProperty private URI url;
    @JsonProperty private String secret;

    public McpEventsSubscribeDeliveryIn() {}

    public McpEventsSubscribeDeliveryIn mode(McpEventsDeliveryMode mode) {
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

    public McpEventsSubscribeDeliveryIn url(URI url) {
        this.url = url;
        return this;
    }

    /**
     * The callback URL events are sent to.
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

    public McpEventsSubscribeDeliveryIn secret(String secret) {
        this.secret = secret;
        return this;
    }

    /**
     * The secret events are signed with. MCP Events requires a `whsec_` secret.
     *
     * @return secret
     */
    @javax.annotation.Nonnull
    public String getSecret() {
        return secret;
    }

    public void setSecret(String secret) {
        this.secret = secret;
    }

    /**
     * Create an instance of McpEventsSubscribeDeliveryIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of McpEventsSubscribeDeliveryIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     McpEventsSubscribeDeliveryIn
     */
    public static McpEventsSubscribeDeliveryIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, McpEventsSubscribeDeliveryIn.class);
    }

    /**
     * Convert an instance of McpEventsSubscribeDeliveryIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
