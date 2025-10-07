// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum ConnectorKind implements ToQueryParam {
    CUSTOM("Custom"),
    AGENTIC_COMMERCE_PROTOCOL("AgenticCommerceProtocol"),
    CLOSE_CRM("CloseCRM"),
    CUSTOMER_IO("CustomerIO"),
    DISCORD("Discord"),
    HUBSPOT("Hubspot"),
    INNGEST("Inngest"),
    LOOPS("Loops"),
    RESEND("Resend"),
    SALESFORCE("Salesforce"),
    SEGMENT("Segment"),
    SENDGRID("Sendgrid"),
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
