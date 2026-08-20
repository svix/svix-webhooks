# this file is @generated
from enum import Enum


class BulkExpungeStatus(str, Enum):
    EXPUNGED = "expunged"
    NOT_FOUND = "not-found"

    def __str__(self) -> str:
        return str(self.value)
