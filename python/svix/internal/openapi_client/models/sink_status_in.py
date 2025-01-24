from enum import Enum


class SinkStatusIn(str, Enum):
    ENABLED = "enabled"
    PAUSED = "paused"

    def __str__(self) -> str:
        return str(self.value)
