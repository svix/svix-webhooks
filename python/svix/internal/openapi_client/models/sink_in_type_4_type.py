from enum import Enum


class SinkInType4Type(str, Enum):
    EVENTSTREAM = "eventStream"

    def __str__(self) -> str:
        return str(self.value)
