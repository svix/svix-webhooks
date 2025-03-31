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
public class ApiTokenExpireIn {
    @JsonProperty private Integer expiry;

    public ApiTokenExpireIn() {}

    public ApiTokenExpireIn expiry(Integer expiry) {
        this.expiry = expiry;
        return this;
    }

    /**
     * How many seconds until the old key is expired.
     *
     * @return expiry
     */
    @javax.annotation.Nullable
    public Integer getExpiry() {
        return expiry;
    }

    public void setExpiry(Integer expiry) {
        this.expiry = expiry;
    }

    /**
     * Create an instance of ApiTokenExpireIn given an JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ApiTokenExpireIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to
     *     ApiTokenExpireIn
     */
    public static ApiTokenExpireIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ApiTokenExpireIn.class);
    }

    /**
     * Convert an instance of ApiTokenExpireIn to an JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
