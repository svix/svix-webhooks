# this file is @generated
from enum import Enum


class Status(str, Enum):
    PENDING = "pending"
    ACTIVE = "active"

    def __str__(self) -> str:
        return str(self.value)
