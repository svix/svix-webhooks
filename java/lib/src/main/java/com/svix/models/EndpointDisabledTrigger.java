// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum EndpointDisabledTrigger implements ToQueryParam {
    MANUAL("manual"),
    AUTOMATIC("automatic");
    private final String value;

    EndpointDisabledTrigger(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }

    @Override
    public String toQueryParam() {
        return this.value;
    }
}
