from enum import Enum


class SinkOutType2Type(str, Enum):
    KAFKA = "kafka"

    def __str__(self) -> str:
        return str(self.value)
