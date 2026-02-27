# this file is @generated
from enum import Enum


class EvictionPolicy(str, Enum):
    NO_EVICTION = "NoEviction"
    LEAST_RECENTLY_USED = "LeastRecentlyUsed"

    def __str__(self) -> str:
        return str(self.value)
