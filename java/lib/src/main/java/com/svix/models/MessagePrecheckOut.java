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
public class MessagePrecheckOut {
    @JsonProperty private Boolean active;

    public MessagePrecheckOut() {}

    public MessagePrecheckOut active(Boolean active) {
        this.active = active;
        return this;
    }

    /**
     * Whether there are any active endpoint that would get sent such a message.
     *
     * @return active
     */
    @javax.annotation.Nonnull
    public Boolean getActive() {
        return active;
    }

    public void setActive(Boolean active) {
        this.active = active;
    }

    /**
     * Create an instance of MessagePrecheckOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MessagePrecheckOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     MessagePrecheckOut
     */
    public static MessagePrecheckOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MessagePrecheckOut.class);
    }

    /**
     * Convert an instance of MessagePrecheckOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
