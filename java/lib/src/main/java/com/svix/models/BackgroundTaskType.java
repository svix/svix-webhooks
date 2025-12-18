// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum BackgroundTaskType implements ToQueryParam {
    ENDPOINT_REPLAY("endpoint.replay"),
    ENDPOINT_RECOVER("endpoint.recover"),
    APPLICATION_STATS("application.stats"),
    MESSAGE_BROADCAST("message.broadcast"),
    SDK_GENERATE("sdk.generate"),
    EVENT_TYPE_AGGREGATE("event-type.aggregate"),
    APPLICATION_PURGE_CONTENT("application.purge_content"),
    ENDPOINT_BULK_REPLAY("endpoint.bulk_replay");
    private final String value;

    BackgroundTaskType(String value) {
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
