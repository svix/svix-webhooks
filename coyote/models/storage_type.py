# this file is @generated
from enum import Enum


class StorageType(str, Enum):
    PERSISTENT = "Persistent"
    EPHEMERAL = "Ephemeral"

    def __str__(self) -> str:
        return str(self.value)
