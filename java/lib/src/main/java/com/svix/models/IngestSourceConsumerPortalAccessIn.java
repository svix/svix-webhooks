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
public class IngestSourceConsumerPortalAccessIn {
    @JsonProperty private Long expiry;
    @JsonProperty private Boolean readOnly;

    public IngestSourceConsumerPortalAccessIn() {}

    public IngestSourceConsumerPortalAccessIn expiry(Long expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
     * How long the token will be valid for, in seconds.
     *
     * <p>Valid values are between 1 hour and 7 days. The default is 7 days.
     *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Long getExpiry() {
        return expiry;
    }

    public void setExpiry(Long expiry) {
        this.expiry = expiry;
    }

    public IngestSourceConsumerPortalAccessIn readOnly(Boolean readOnly) {
        this.readOnly = readOnly;
        return this;
    }

    /**
     * Whether the app portal should be in read-only mode.
     *
     * @return readOnly
     */
    @javax.annotation.Nullable
    public Boolean getReadOnly() {
        return readOnly;
    }

    public void setReadOnly(Boolean readOnly) {
        this.readOnly = readOnly;
    }

    /**
     * Create an instance of IngestSourceConsumerPortalAccessIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of IngestSourceConsumerPortalAccessIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     IngestSourceConsumerPortalAccessIn
     */
    public static IngestSourceConsumerPortalAccessIn fromJson(String jsonString)
            throws JsonProcessingException {
        return Utils.getObjectMapper()
                .readValue(jsonString, IngestSourceConsumerPortalAccessIn.class);
    }

    /**
     * Convert an instance of IngestSourceConsumerPortalAccessIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
