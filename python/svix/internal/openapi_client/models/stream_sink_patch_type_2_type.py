from enum import Enum


class StreamSinkPatchType2Type(str, Enum):
    HTTP = "http"

    def __str__(self) -> str:
        return str(self.value)
