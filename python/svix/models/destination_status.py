# this file is @generated
from enum import Enum


class DestinationStatus(str, Enum):
    ENABLED = "enabled"
    PAUSED = "paused"
    DISABLED = "disabled"
    RETRYING = "retrying"

    def __str__(self) -> str:
        return str(self.value)
