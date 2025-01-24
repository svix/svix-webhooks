from enum import Enum


class SinkStatus(str, Enum):
    DISABLED = "disabled"
    ENABLED = "enabled"
    PAUSED = "paused"

    def __str__(self) -> str:
        return str(self.value)
