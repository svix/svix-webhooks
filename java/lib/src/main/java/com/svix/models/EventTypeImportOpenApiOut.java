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
public class EventTypeImportOpenApiOut {
    @JsonProperty private EventTypeImportOpenApiOutData data;

    public EventTypeImportOpenApiOut() {}

    public EventTypeImportOpenApiOut data(EventTypeImportOpenApiOutData data) {
        this.data = data;
        return this;
    }

    /**
     * Get data
     *
     * @return data
     */
    @javax.annotation.Nonnull
    public EventTypeImportOpenApiOutData getData() {
        return data;
    }

    public void setData(EventTypeImportOpenApiOutData data) {
        this.data = data;
    }

    /**
     * Create an instance of EventTypeImportOpenApiOut given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeImportOpenApiOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventTypeImportOpenApiOut
     */
    public static EventTypeImportOpenApiOut fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeImportOpenApiOut.class);
    }

    /**
     * Convert an instance of EventTypeImportOpenApiOut to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
