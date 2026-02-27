# this file is @generated
from enum import Enum


class OperationBehavior(str, Enum):
    UPSERT = "upsert"
    INSERT = "insert"
    UPDATE = "update"

    def __str__(self) -> str:
        return str(self.value)
