# this file is @generated
from enum import Enum


class Consistency(str, Enum):
    """Consistency level for reads.

    Strong consistency (also known as linearizability) guarantees that a read will see all previous
    writes. Weak consistency allows stale reads, but can save one or more round trip to the leader."""

    STRONG = "strong"
    WEAK = "weak"

    def __str__(self) -> str:
        return str(self.value)
