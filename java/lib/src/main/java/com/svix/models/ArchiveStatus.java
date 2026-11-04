// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum ArchiveStatus implements ToQueryParam {
    ACTIVE("active"),
    ARCHIVED("archived"),
    ALL("all");
    private final String value;

    ArchiveStatus(String value) {
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
