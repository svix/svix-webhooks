from enum import Enum


class StreamSinkOutType6Type(str, Enum):
    REDSHIFT = "redshift"

    def __str__(self) -> str:
        return str(self.value)
