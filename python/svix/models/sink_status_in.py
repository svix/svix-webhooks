# this file is @generated
from enum import Enum


class SinkStatusIn(str, Enum):
    ENABLED = "enabled"
    DISABLED = "disabled"

    def __str__(self) -> str:
        return str(self.value)
