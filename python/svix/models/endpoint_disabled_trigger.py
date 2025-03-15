# this file is @generated
from enum import Enum


class EndpointDisabledTrigger(str, Enum):
    MANUAL = "manual"
    AUTOMATIC = "automatic"

    def __str__(self) -> str:
        return str(self.value)
