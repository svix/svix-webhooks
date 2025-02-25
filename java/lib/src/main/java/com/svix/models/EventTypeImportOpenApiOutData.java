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
public class EventTypeImportOpenApiOutData {
    @JsonProperty private List<String> modified;

    @JsonProperty("to_modify")
    private List<EventTypeFromOpenApi> toModify;

    public EventTypeImportOpenApiOutData() {}

    public EventTypeImportOpenApiOutData modified(List<String> modified) {
        this.modified = modified;
        return this;
    }

    public EventTypeImportOpenApiOutData addModifiedItem(String modifiedItem) {
        if (this.modified == null) {
            this.modified = new ArrayList<>();
        }
        this.modified.add(modifiedItem);

        return this;
    }

    /**
     * Get modified
     *
     * @return modified
     */
    @javax.annotation.Nonnull
    public List<String> getModified() {
        return modified;
    }

    public void setModified(List<String> modified) {
        this.modified = modified;
    }

    public EventTypeImportOpenApiOutData toModify(List<EventTypeFromOpenApi> toModify) {
        this.toModify = toModify;
        return this;
    }

    public EventTypeImportOpenApiOutData addToModifyItem(EventTypeFromOpenApi toModifyItem) {
        if (this.toModify == null) {
            this.toModify = new ArrayList<>();
        }
        this.toModify.add(toModifyItem);

        return this;
    }

    /**
     * Get toModify
     *
     * @return toModify
     */
    @javax.annotation.Nullable
    public List<EventTypeFromOpenApi> getToModify() {
        return toModify;
    }

    public void setToModify(List<EventTypeFromOpenApi> toModify) {
        this.toModify = toModify;
    }

    /**
     * Create an instance of EventTypeImportOpenApiOutData given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeImportOpenApiOutData
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventTypeImportOpenApiOutData
     */
    public static EventTypeImportOpenApiOutData fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeImportOpenApiOutData.class);
    }

    /**
     * Convert an instance of EventTypeImportOpenApiOutData to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
