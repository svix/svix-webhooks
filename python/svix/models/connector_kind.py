# this file is @generated
from enum import Enum


class ConnectorKind(str, Enum):
    CUSTOM = "Custom"
    AGENTIC_COMMERCE_PROTOCOL = "AgenticCommerceProtocol"
    CLOSE_CRM = "CloseCRM"
    CUSTOMER_IO = "CustomerIO"
    DISCORD = "Discord"
    HUBSPOT = "Hubspot"
    INNGEST = "Inngest"
    LOOPS = "Loops"
    OTEL = "Otel"
    RESEND = "Resend"
    SALESFORCE = "Salesforce"
    SEGMENT = "Segment"
    SENDGRID = "Sendgrid"
    SLACK = "Slack"
    TEAMS = "Teams"
    TRIGGER_DEV = "TriggerDev"
    WINDMILL = "Windmill"
    ZAPIER = "Zapier"

    def __str__(self) -> str:
        return str(self.value)
