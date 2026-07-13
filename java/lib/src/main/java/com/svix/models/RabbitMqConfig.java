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
public class RabbitMqConfig {
    @JsonProperty private String uri;
    @JsonProperty private String routingKey;

    public RabbitMqConfig() {}

    public RabbitMqConfig uri(String uri) {
        this.uri = uri;
        return this;
    }

    /**
     * Get uri
     *
     * @return uri
     */
    @javax.annotation.Nonnull
    public String getUri() {
        return uri;
    }

    public void setUri(String uri) {
        this.uri = uri;
    }

    public RabbitMqConfig routingKey(String routingKey) {
        this.routingKey = routingKey;
        return this;
    }

    /**
     * Get routingKey
     *
     * @return routingKey
     */
    @javax.annotation.Nonnull
    public String getRoutingKey() {
        return routingKey;
    }

    public void setRoutingKey(String routingKey) {
        this.routingKey = routingKey;
    }

    /**
     * Create an instance of RabbitMqConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RabbitMqConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to RabbitMqConfig
     */
    public static RabbitMqConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RabbitMqConfig.class);
    }

    /**
     * Convert an instance of RabbitMqConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
