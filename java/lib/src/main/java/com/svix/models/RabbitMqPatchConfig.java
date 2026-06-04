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
public class RabbitMqPatchConfig {
    @JsonProperty private String routingKey;
    @JsonProperty private String uri;

    public RabbitMqPatchConfig() {}

    public RabbitMqPatchConfig routingKey(String routingKey) {
        this.routingKey = routingKey;
        return this;
    }

    /**
     * Get routingKey
     *
     * @return routingKey
     */
    @javax.annotation.Nullable
    public String getRoutingKey() {
        return routingKey;
    }

    public void setRoutingKey(String routingKey) {
        this.routingKey = routingKey;
    }

    public RabbitMqPatchConfig uri(String uri) {
        this.uri = uri;
        return this;
    }

    /**
     * Get uri
     *
     * @return uri
     */
    @javax.annotation.Nullable
    public String getUri() {
        return uri;
    }

    public void setUri(String uri) {
        this.uri = uri;
    }

    /**
     * Create an instance of RabbitMqPatchConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RabbitMqPatchConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RabbitMqPatchConfig
     */
    public static RabbitMqPatchConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RabbitMqPatchConfig.class);
    }

    /**
     * Convert an instance of RabbitMqPatchConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
