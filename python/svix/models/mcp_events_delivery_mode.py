# this file is @generated
from enum import Enum


class McpEventsDeliveryMode(str, Enum):
    WEBHOOK = "webhook"

    def __str__(self) -> str:
        return str(self.value)
