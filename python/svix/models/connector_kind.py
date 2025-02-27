# this file is @generated
from enum import Enum


class ConnectorKind(str, Enum):
    CUSTOM = "Custom"
    CUSTOMER_IO = "CustomerIO"
    DISCORD = "Discord"
    HUBSPOT = "Hubspot"
    INNGEST = "Inngest"
    SALESFORCE = "Salesforce"
    SEGMENT = "Segment"
    SLACK = "Slack"
    TEAMS = "Teams"
    TRIGGER_DEV = "TriggerDev"
    WINDMILL = "Windmill"
    ZAPIER = "Zapier"

    def __str__(self) -> str:
        return str(self.value)
