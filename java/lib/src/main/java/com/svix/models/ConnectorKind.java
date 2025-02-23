// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum ConnectorKind implements ToQueryParam {
    CUSTOM("Custom"),
    CUSTOMER_IO("CustomerIO"),
    DISCORD("Discord"),
    HUBSPOT("Hubspot"),
    INNGEST("Inngest"),
    SALESFORCE("Salesforce"),
    SEGMENT("Segment"),
    SLACK("Slack"),
    TEAMS("Teams"),
    TRIGGER_DEV("TriggerDev"),
    WINDMILL("Windmill"),
    ZAPIER("Zapier");
    private final String value;

    ConnectorKind(String value) {
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
