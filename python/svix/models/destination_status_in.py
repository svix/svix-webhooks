# this file is @generated
from enum import Enum


class DestinationStatusIn(str, Enum):
    ENABLED = "enabled"
    DISABLED = "disabled"

    def __str__(self) -> str:
        return str(self.value)
