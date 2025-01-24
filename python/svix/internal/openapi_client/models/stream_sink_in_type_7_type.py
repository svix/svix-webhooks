from enum import Enum


class StreamSinkInType7Type(str, Enum):
    BIGQUERY = "bigQuery"

    def __str__(self) -> str:
        return str(self.value)
