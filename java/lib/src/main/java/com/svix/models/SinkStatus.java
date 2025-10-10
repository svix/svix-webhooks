// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum SinkStatus implements ToQueryParam {
    ENABLED("enabled"),
    PAUSED("paused"),
    DISABLED("disabled"),
    RETRYING("retrying");
    private final String value;

    SinkStatus(String value) {
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
