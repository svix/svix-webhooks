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

import java.util.ArrayList;
import java.util.List;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class BatchCreateMessagesIn {
    @JsonProperty private List<BatchMessageIn> messages;

    public BatchCreateMessagesIn() {}

    public BatchCreateMessagesIn messages(List<BatchMessageIn> messages) {
        this.messages = messages;
        return this;
    }

    public BatchCreateMessagesIn addMessagesItem(BatchMessageIn messagesItem) {
        if (this.messages == null) {
            this.messages = new ArrayList<>();
        }
        this.messages.add(messagesItem);

        return this;
    }

    /**
     * Get messages
     *
     * @return messages
     */
    @javax.annotation.Nonnull
    public List<BatchMessageIn> getMessages() {
        return messages;
    }

    public void setMessages(List<BatchMessageIn> messages) {
        this.messages = messages;
    }

    /**
     * Create an instance of BatchCreateMessagesIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BatchCreateMessagesIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     BatchCreateMessagesIn
     */
    public static BatchCreateMessagesIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BatchCreateMessagesIn.class);
    }

    /**
     * Convert an instance of BatchCreateMessagesIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
