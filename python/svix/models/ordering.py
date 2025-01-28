# this file is @generated
from enum import Enum


class Ordering(str, Enum):
    """Defines the ordering in a listing of results."""

    ASCENDING = "ascending"
    DESCENDING = "descending"

    def __str__(self) -> str:
        return str(self.value)
