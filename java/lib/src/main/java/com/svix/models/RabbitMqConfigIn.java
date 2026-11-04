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
public class RabbitMqConfigIn {
    @JsonProperty private String uri;
    @JsonProperty private String routingKey;
    @JsonProperty private Boolean mandatory;

    public RabbitMqConfigIn() {}

    public RabbitMqConfigIn uri(String uri) {
        this.uri = uri;
        return this;
    }

    /**
     * URI to connect to
     *
     * <p>Note that the VHost must be percent-escaped, so a default URI would look like
     * `amqp://user:pass@host/%2F`
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

    public RabbitMqConfigIn routingKey(String routingKey) {
        this.routingKey = routingKey;
        return this;
    }

    /**
     * Routing key for message dispatch
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

    public RabbitMqConfigIn mandatory(Boolean mandatory) {
        this.mandatory = mandatory;
        return this;
    }

    /**
     * If true, then dispatches will fail if there is no attached queue; if false, they are silently
     * dropped (this was previously the default)
     *
     * @return mandatory
     */
    @javax.annotation.Nullable
    public Boolean getMandatory() {
        return mandatory;
    }

    public void setMandatory(Boolean mandatory) {
        this.mandatory = mandatory;
    }

    /**
     * Create an instance of RabbitMqConfigIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of RabbitMqConfigIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     RabbitMqConfigIn
     */
    public static RabbitMqConfigIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, RabbitMqConfigIn.class);
    }

    /**
     * Convert an instance of RabbitMqConfigIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
