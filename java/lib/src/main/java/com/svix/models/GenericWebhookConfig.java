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
public class GenericWebhookConfig {
    @JsonProperty private GenericWebhookResponse response;

    public GenericWebhookConfig() {}

    public GenericWebhookConfig response(GenericWebhookResponse response) {
        this.response = response;
        return this;
    }

    /**
     * Get response
     *
     * @return response
     */
    @javax.annotation.Nullable
    public GenericWebhookResponse getResponse() {
        return response;
    }

    public void setResponse(GenericWebhookResponse response) {
        this.response = response;
    }

    /**
     * Create an instance of GenericWebhookConfig given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of GenericWebhookConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     GenericWebhookConfig
     */
    public static GenericWebhookConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, GenericWebhookConfig.class);
    }

    /**
     * Convert an instance of GenericWebhookConfig to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
