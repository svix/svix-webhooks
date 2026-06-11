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
public class SubscribeIn {
    @JsonProperty private EndpointIn endpoint;
    @JsonProperty private AutoConfigSinkType sink;

    public SubscribeIn() {}

    public SubscribeIn endpoint(EndpointIn endpoint) {
        this.endpoint = endpoint;
        return this;
    }

    /**
     * Get endpoint
     *
     * @return endpoint
     */
    @javax.annotation.Nullable
    public EndpointIn getEndpoint() {
        return endpoint;
    }

    public void setEndpoint(EndpointIn endpoint) {
        this.endpoint = endpoint;
    }

    public SubscribeIn sink(AutoConfigSinkType sink) {
        this.sink = sink;
        return this;
    }

    /**
     * Get sink
     *
     * @return sink
     */
    @javax.annotation.Nullable
    public AutoConfigSinkType getSink() {
        return sink;
    }

    public void setSink(AutoConfigSinkType sink) {
        this.sink = sink;
    }

    /**
     * Create an instance of SubscribeIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of SubscribeIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to SubscribeIn
     */
    public static SubscribeIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SubscribeIn.class);
    }

    /**
     * Convert an instance of SubscribeIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
