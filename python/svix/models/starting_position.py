# this file is @generated
from enum import Enum


class StartingPosition(str, Enum):
    EARLIEST = "earliest"
    LATEST = "latest"

    def __str__(self) -> str:
        return str(self.value)
