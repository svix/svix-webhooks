from enum import Enum


class SinkInType2Type(str, Enum):
    KAFKA = "kafka"

    def __str__(self) -> str:
        return str(self.value)
