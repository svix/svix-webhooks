from enum import Enum


class SinkOutType4Type(str, Enum):
    EVENTSTREAM = "eventStream"

    def __str__(self) -> str:
        return str(self.value)
