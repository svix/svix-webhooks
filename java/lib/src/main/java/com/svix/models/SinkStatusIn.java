// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum SinkStatusIn implements ToQueryParam {
    ENABLED("enabled"),
    DISABLED("disabled");
    private final String value;

    SinkStatusIn(String value) {
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
