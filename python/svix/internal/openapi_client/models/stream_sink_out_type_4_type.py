from enum import Enum


class StreamSinkOutType4Type(str, Enum):
    SNOWFLAKE = "snowflake"

    def __str__(self) -> str:
        return str(self.value)
