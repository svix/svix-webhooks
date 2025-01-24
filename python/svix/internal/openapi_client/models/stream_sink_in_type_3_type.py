from enum import Enum


class StreamSinkInType3Type(str, Enum):
    AMAZONS3 = "amazonS3"

    def __str__(self) -> str:
        return str(self.value)
