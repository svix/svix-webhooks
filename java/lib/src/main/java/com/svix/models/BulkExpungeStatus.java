// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum BulkExpungeStatus implements ToQueryParam {
    EXPUNGED("expunged"),
    NOT_FOUND("not-found");
    private final String value;

    BulkExpungeStatus(String value) {
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
