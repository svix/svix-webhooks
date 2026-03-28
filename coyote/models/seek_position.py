# this file is @generated
from enum import Enum


class SeekPosition(str, Enum):
    EARLIEST = "earliest"
    LATEST = "latest"

    def __str__(self) -> str:
        return str(self.value)
