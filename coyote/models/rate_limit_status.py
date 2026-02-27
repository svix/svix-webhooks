# this file is @generated
from enum import Enum


class RateLimitStatus(str, Enum):
    OK = "ok"
    BLOCK = "block"

    def __str__(self) -> str:
        return str(self.value)
