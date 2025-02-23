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
public class EventTypeImportOpenApiIn {
    @JsonProperty private Boolean dryRun;
    @JsonProperty private Boolean replaceAll;
    @JsonProperty private Object spec;
    @JsonProperty private String specRaw;

    public EventTypeImportOpenApiIn() {}

    public EventTypeImportOpenApiIn dryRun(Boolean dryRun) {
        this.dryRun = dryRun;
        return this;
    }

    /**
     * If `true`, return the event types that would be modified without actually modifying them.
     *
     * @return dryRun
     */
    @javax.annotation.Nullable
    public Boolean getDryRun() {
        return dryRun;
    }

    public void setDryRun(Boolean dryRun) {
        this.dryRun = dryRun;
    }

    public EventTypeImportOpenApiIn replaceAll(Boolean replaceAll) {
        this.replaceAll = replaceAll;
        return this;
    }

    /**
     * If `true`, all existing event types that are not in the spec will be archived.
     *
     * @return replaceAll
     */
    @javax.annotation.Nullable
    public Boolean getReplaceAll() {
        return replaceAll;
    }

    public void setReplaceAll(Boolean replaceAll) {
        this.replaceAll = replaceAll;
    }

    public EventTypeImportOpenApiIn spec(Object spec) {
        this.spec = spec;
        return this;
    }

    /**
     * A pre-parsed JSON spec.
     *
     * @return spec
     */
    @javax.annotation.Nullable
    public Object getSpec() {
        return spec;
    }

    public void setSpec(Object spec) {
        this.spec = spec;
    }

    public EventTypeImportOpenApiIn specRaw(String specRaw) {
        this.specRaw = specRaw;
        return this;
    }

    /**
     * A string, parsed by the server as YAML or JSON.
     *
     * @return specRaw
     */
    @javax.annotation.Nullable
    public String getSpecRaw() {
        return specRaw;
    }

    public void setSpecRaw(String specRaw) {
        this.specRaw = specRaw;
    }

    /**
     * Create an instance of EventTypeImportOpenApiIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of EventTypeImportOpenApiIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     EventTypeImportOpenApiIn
     */
    public static EventTypeImportOpenApiIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, EventTypeImportOpenApiIn.class);
    }

    /**
     * Convert an instance of EventTypeImportOpenApiIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
