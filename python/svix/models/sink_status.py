# this file is @generated
from enum import Enum


class SinkStatus(str, Enum):
    ENABLED = "enabled"
    PAUSED = "paused"
    DISABLED = "disabled"
    RETRYING = "retrying"

    def __str__(self) -> str:
        return str(self.value)
