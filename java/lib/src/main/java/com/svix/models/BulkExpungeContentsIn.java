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

import java.util.LinkedHashSet;
import java.util.Set;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class BulkExpungeContentsIn {
    @JsonProperty private Set<String> ids;

    public BulkExpungeContentsIn() {}

    public BulkExpungeContentsIn ids(Set<String> ids) {
        this.ids = ids;
        return this;
    }

    public BulkExpungeContentsIn addIdsItem(String idsItem) {
        if (this.ids == null) {
            this.ids = new LinkedHashSet<>();
        }
        this.ids.add(idsItem);

        return this;
    }

    /**
     * Message ID or UID to delete
     *
     * @return ids
     */
    @javax.annotation.Nullable
    public Set<String> getIds() {
        return ids;
    }

    public void setIds(Set<String> ids) {
        this.ids = ids;
    }

    /**
     * Create an instance of BulkExpungeContentsIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of BulkExpungeContentsIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     BulkExpungeContentsIn
     */
    public static BulkExpungeContentsIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, BulkExpungeContentsIn.class);
    }

    /**
     * Convert an instance of BulkExpungeContentsIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
