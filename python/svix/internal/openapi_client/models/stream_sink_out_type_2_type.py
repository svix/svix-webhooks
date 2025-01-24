from enum import Enum


class StreamSinkOutType2Type(str, Enum):
    HTTP = "http"

    def __str__(self) -> str:
        return str(self.value)
