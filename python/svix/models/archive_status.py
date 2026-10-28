# this file is @generated
from enum import Enum


class ArchiveStatus(str, Enum):
    ACTIVE = "active"
    ARCHIVED = "archived"
    ALL = "all"

    def __str__(self) -> str:
        return str(self.value)
