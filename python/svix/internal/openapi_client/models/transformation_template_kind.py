from enum import Enum


class TransformationTemplateKind(str, Enum):
    CUSTOM = "Custom"
    CUSTOMERIO = "CustomerIO"
    DISCORD = "Discord"
    HUBSPOT = "Hubspot"
    INNGEST = "Inngest"
    SALESFORCE = "Salesforce"
    SEGMENT = "Segment"
    SLACK = "Slack"
    TEAMS = "Teams"
    TRIGGERDEV = "TriggerDev"
    WINDMILL = "Windmill"
    ZAPIER = "Zapier"

    def __str__(self) -> str:
        return str(self.value)
